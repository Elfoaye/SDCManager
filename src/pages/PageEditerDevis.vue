<script setup>
import { invoke } from '@tauri-apps/api/core';
import { ref, computed ,onMounted, watch } from 'vue';
import { useBreadcrumb } from '../composables/breadcrumb';
import { useDevisStore } from '../composables/devisStore';
import ListeSelectionDevis from '../components/ListeSelectionDevis.vue';

const props = defineProps(['document', 'setDocument']);

const { setBreadcrumb } = useBreadcrumb();
setBreadcrumb([
    { label: 'Accueil', page: null },
    { label: 'Devis & Factures', page: 'devparcour' },
    { label: 'Éditer', page: 'devmodif' }
]);

const store = useDevisStore();
const tempExtrafield = ref({
    name: '',
    price: ''
});

const saveMassage = ref({ result: 'success', message: '' });
const loadError = ref('');
const confirmExtra = ref('');

const notDispoItems = ref([]);

const formulas = ref(null);
const contextName = ref('');

const displaySelected = ref(true);

function addExtrafield(){
    if(tempExtrafield.value.name === '') return;

    if (tempExtrafield.value.price === '') {
        tempExtrafield.value.price = 0;
    }

    store.extraItems.push(tempExtrafield.value);
    tempExtrafield.value = {name: '', price: ''};
}

function removeExtraField(item) {
    const index = store.extraItems.indexOf(item);
    if (index !== -1) {
        store.extraItems.splice(index, 1);
    }
    confirmExtra.value = '';
}

function confirmRemoveExtra(item) {
    confirmExtra.value = item;
}

const finalCost = computed(() => {
    let price = store.utilitaries.techQty * store.utilitaries.techRate;
    price += store.utilitaries.transportKm * store.utilitaries.transportRate;
    if(store.utilitaries.membership && formulas.value) {
        price += formulas.value.membership;
    }

    price += store.selectedItems.reduce((sum, item) => sum + item.totalPrice, 0);
    price += store.extraItems.reduce((sum, item) => sum + item.price, 0);

    price -= store.utilitaries.discountEuro;

    return price;
});

const uniqueClientNames = computed(() => {
    if(!store.clients) return [];
    
    const names = store.clients.map(c => c.nom);
    return [...new Set(names)];
});

const matchingEvents = computed(() => {
    if(!store.clients) return [];

    return store.clients
        .filter(c => c.nom === store.clientInfos.name)
        .map(c => c.evenement);
});

function onNameInput() {
    store.clientInfos.eventName = '';
    store.clientInfos.adress = '';
    store.clientInfos.phone = '';
    store.clientInfos.mail = '';
}

function onEventInput() {
    if(!store.clients) return;

    const client = store.clients.find(c =>
        c.nom === store.clientInfos.name && c.evenement === store.clientInfos.eventName
    );
    if (client) {
        store.clientInfos.id = client.id;
        store.clientInfos.adress = client.adresse;
        store.clientInfos.phone = client.tel;
        store.clientInfos.mail = client.mail;
    }
}

function setTechRate() {
    store.utilitaries.techRate = store.utilitaries.techHourly ? formulas.value.tech_hour : formulas.value.tech_day;
}

function setContextName() {
    if(store.devisInfos.id <= 0) {
        contextName.value = '';
        return;
    }

    contextName.value = store.devisInfos.id + ' ' + store.devisInfos.name;
}

function newDevis() {
    store.reset();
    if(props.document !== 0) props.setDocument({id: 0, facture: false}, true);

    setTechRate();
    store.utilitaries.transportRate = formulas.value.transport_km;

    saveMassage.value = null;
    setContextName();
}

async function loadDocument(document) {
    if(document.facture || document.id <= 0) {
        newDevis();
        setContextName();
        return;
    }

    try {
        await store.loadDocument(document);
        loadError.value = '';
        setContextName();
    } catch (err) {
        console.error(err + " on loading document " + document.id);
        loadError.value = err;
    }
}

function cancelDevis() {
    loadDocument({id: store.devisInfos.id, facture: false});
}

async function endModif() {
    try {
        await store.saveDevis('devis');
        props.setDocument({id: store.devisInfos.id, facture: false}, false);
    } catch(err) {
        console.error("Erreur lors de la sauvegarde du devis : ", err);
        saveMassage.value = err;
    }
}

async function checkDispo(item) {
    if(!store.devisInfos.date || !item.duration || !item.quantity)
        return true;

    try {
        const dispo = await invoke('get_item_dispo', { id: item.id, date: store.devisInfos.date, duration: item.duration})
        if(item.quantity > dispo) {
            notDispoItems.value.push({item, dispo})
        }
    } catch (err) {
        console.error("Error while checking dispo of " + item.nom, err);
    }
    
}

function checkAllSelected() {
    store.selectedItems.forEach((item) => {
        checkDispo(item);
    })
}

function clearNotDispo() {
    notDispoItems.value = [];
}

function adjustNotDispo() {
    notDispoItems.value.forEach((element) => {
        store.setItem(element.item, element.dispo, 'unset');
    });
    clearNotDispo();
}

onMounted(async() => {
    formulas.value = await invoke('get_loc_formulas');
    saveMassage.value = null;

    loadDocument(props.document);
    
    if(store.utilitaries.techRate === 0) {
        setTechRate();
    }
    if(store.utilitaries.transportRate === 0) {
        store.utilitaries.transportRate = formulas.value.transport_km;
    }
});

watch(() => store.utilitaries.techHourly, () => {
    setTechRate();
});

let lastValidDuration = store.devisInfos.duration;
watch(() => store.devisInfos.duration, (newVal) => {
    if (!newVal || newVal <= 0) return;

    store.selectedItems.forEach(item => {
        if(item.duration === lastValidDuration) {
            store.setItem(item, 'unset', newVal);
        }
    });

    lastValidDuration = newVal;
});

watch(() => [store.devisInfos.date, store.devisInfos.duration], () => {
    if (store.selectedItems.length > 0) {
        checkAllSelected();
    }
});
</script>

<template>
    <div class="content">
        <div v-if="notDispoItems.length > 0" class="confirm">
            <div class="pop-up">
                <p>Les objets suivants nes sont pas disponible sur la periode sélectinnée :</p>
                <div class="not-dispo-holder">
                    <p v-for="item in notDispoItems" class="not-dispo">{{ item.item.nom }}</p>
                </div>
                <p>Voulez-vous ajuster la quantité ?</p>

                <div class="confirm-buttons">
                    <button @click="adjustNotDispo" class="new" >Ajuster</button>
                    <button @click="clearNotDispo" class="cancel">Ignorer</button>
                </div>
            </div>
        </div>
        <div class="title" :class="{ new: store.devisInfos.id === 0 }">
            <h1 v-if="store.devisInfos.id > 0">Éditer le devis</h1>
            <h1 v-else>Nouveau devis</h1>
        </div>
        <p v-if="loadError" class="error">{{ loadError }}</p>
        <div class="context" v-if="contextName">
            <h2>{{ contextName }}</h2>
        </div>
        <div class="content-parts">
            <div class="part">
                <section class="infos">
                    <h2>Informations générales</h2>
                    <div class="line tech">
                        <label>Nom du devis :
                            <input v-model="store.devisInfos.name"/>
                        </label>
                        <label>Date de début:
                            <input type="date" v-model="store.devisInfos.date"/>
                        </label>
                        <label>Durée (Jours) :
                            <input type="number" v-model="store.devisInfos.duration" min="1"/>
                        </label>
                    </div>
                </section>
                <section class="infos">
                    <h2>Client</h2>
                    <div class="line">
                        <label>Nom du client :
                            <input v-model="store.clientInfos.name" list="client-names" @input="onNameInput"/>
                            <datalist id="client-names">
                                <option v-for="name in uniqueClientNames" :key="name">{{ name }}</option>
                            </datalist>
                        </label>
                        <label>Evenement :
                            <input v-model="store.clientInfos.eventName" list="event-names" @input="onEventInput"/>
                            <datalist id="event-names">
                                <option v-for="event in matchingEvents" :key="event">{{ event }}</option>
                            </datalist>
                        </label>
                        <label>Addresse du client :
                            <textarea v-model="store.clientInfos.adress"></textarea>
                        </label>
                    </div>
                    <div class="line">
                        <label>Mail du client :
                            <input v-model="store.clientInfos.mail"/>
                        </label>
                        <label>Telephone du client :
                            <input v-model="store.clientInfos.phone"/>
                        </label>
                    </div>
                    <p class="note">note: Seuls le nom et l'adresse apparaissent sur le document, l'evenement sert à retrouver l'adresse plus facilement</p>
                </section>
                <section class="base">
                    <h2>Utilitaires</h2>
                    <div class="line tech">
                        <label>Techniciens :
                            <input type="number" v-model="store.utilitaries.techQty" min="0"/>
                        </label>
                        <div class="rate">
                            <label>Prix unitaire :
                                <input type="number" v-model="store.utilitaries.techRate" min="0"/>
                            </label>
                            <label class="inline check">Par Heure ?
                                <input type="checkbox" v-model="store.utilitaries.techHourly"/>
                            </label>
                        </div>
                        <label>Total : 
                            <span>{{ (store.utilitaries.techQty * store.utilitaries.techRate).toFixed(2) }} €</span>
                        </label>
                    </div>
                    <div class="line transport">
                        <label>Transport (km):
                            <input type="number" v-model="store.utilitaries.transportKm" min="0"/>
                        </label>
                        <label>Prix unitaire :
                            <input type="number" v-model="store.utilitaries.transportRate" min="0" step="0.1"/>
                        </label>
                        <label>Total : 
                            <span>{{ (store.utilitaries.transportKm * store.utilitaries.transportRate).toFixed(2) }} €</span>
                        </label>
                    </div>
                    <div class="line membership">
                        <label class="inline check">Adhésion ?
                            <input type="checkbox" v-model="store.utilitaries.membership"/>
                        </label>
                    </div>
                </section>
                
                <section class="bonus">
                    <div v-if="confirmExtra" class="confirm">
                        <div class="pop-up">
                            <p>Êtes vous sûr de vouloir supprimer {{ confirmExtra.name }} ?</p>

                            <div class="confirm-buttons">
                                <button @click="removeExtraField(confirmExtra)" class="delete" >Supprimer</button>
                                <button @click="confirmExtra = ''" class="cancel">Annuler</button>
                            </div>
                        </div>
                    </div>
                    <h2>Autre</h2>
                    <div class="other">
                        <label>Nom : 
                            <textarea v-model="tempExtrafield.name"></textarea>
                        </label>
                        
                        <label>Prix (€) : 
                            <input type="number" v-model="tempExtrafield.price"/>
                        </label>
                        <button class="extra-button" @click="addExtrafield">Ajouter</button>
                    </div>
                    <div class="free">
                        <label>Geste commercial (€) : 
                            <input type="number" v-model="store.utilitaries.discountEuro"/>
                        </label>
                    </div>
                    <div v-if="store.extraItems.length > 0">
                        <h3>Extras :</h3>
                        <ul  v-if="store.extraItems.length > 0">
                            <li class="extra" v-for="item in store.extraItems" :data-id="item.id">
                                <p style="white-space: pre-line;">{{ item.name }}</p>
                                <p>{{ item.price.toFixed(2) }} €</p>
                                <button class="extra-button" @click="confirmRemoveExtra(item)">Supprimer</button>
                            </li>
                        </ul>
                    </div>
                </section>
            </div>
            <div class="part">
                <section class="materiel">
                    <h2>Materiel</h2>
                    <ListeSelectionDevis 
                        v-if="store.devisInfos.date && store.devisInfos.duration" 
                        class="select-list" 
                        @item-updated="checkDispo"
                    />
                    <p v-else>Entrez une date de début et une durée au devis pour ajouter du materiel</p>

                    <div v-if="store.selectedItems.length > 0">
                        <h3 class="toggle-materiel" @click="displaySelected = !displaySelected">
                            <span v-if="displaySelected">&#9660;</span>
                            <span v-else>&#9654;</span>
                            Materiel selectionné
                        </h3>
                        <ul v-if="displaySelected">
                            <li v-for="item in store.selectedItems" :data-id="item.id">
                                <p>{{ item.nom }}</p>
                                <p>{{ item.contrib.toFixed(2) }} €</p>
                                <p>{{ item.quantity }}</p>
                                <p>{{ item.duration }}</p>
                                <p>{{ item.totalPrice.toFixed(2) }} €</p>
                            </li>
                        </ul>
                    </div>
                </section>
            </div>
        </div>
            <section class="total">
                <h2><span>Prix total : {{ finalCost.toFixed(2) }} €</span></h2>
            </section>
            
            <section class="submit">
                <div class="buttons">
                    <button @click="endModif" class="new">
                        Terminer
                    </button>
                    <button @click="cancelDevis" class="cancel">
                        Annuler
                    </button>
                </div>
                <p v-if="saveMassage" :class="saveMassage.result">{{ saveMassage.message }}</p>
            </section>
    </div>
</template>

<style scoped>
.content {
    align-items: center;
    width: 100%;
    max-width: 100%;
    gap: 1rem 5rem;
}

.content-parts {
    display: flex;
    gap: 1rem;
    flex-wrap: wrap;
    justify-content: center;
    width: 100%;
}

.part {
    display: flex;
    flex-direction: column;
    width: 100%;
    max-width: 60rem;
    gap: 1rem;
}

.title {
    width: 60rem;
    max-width: 95%;
}

.confirm {
    position: absolute;
    top: 0;
    left: 0;
    right: 0;
    bottom: 0;
    display: flex;
    justify-content: center;
    align-items: center;
    background-color: rgba(0, 0, 0, 0.5);
    z-index: 1000;
}

.pop-up {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    height: fit-content;
    max-height: 95%;
    max-width: 95%;
    background-color: var(--background-alt);
    border: 1px solid var(--border-accent);
    border-radius: 0.5rem;
    padding: 1rem;
    padding-bottom: 2rem;
}

.confirm-buttons {
    display: flex;
    gap: 1rem;
}

.not-dispo-holder {
    display: flex;
    gap: 1rem;
}

.not-dispo {
    margin: 0;
}

.title {
    border: 2px solid var(--warning);
}

.title.new {
    border: 2px solid var(--success);
}

.context {
    display: flex;
    justify-self: flex-start;
    margin: 0;
}

section {
    padding: 1rem;
    border: 1px solid var(--border);
    border-radius: 0.5rem;
}

section h2 {
    margin-bottom: 1rem;
}

section.base {
    display: flex;
    flex-direction: column;
    gap: 1rem;
}

h3 {
    margin-top: 2rem;
}

.note {
    color: var(--border);
    font-size: 10;
}

.line {
    display: flex;
    align-items: start;
    gap: 1rem;
    width: 100%;
}

.line input {
    flex: 1;
    width: 95%;
}

.line.tech, .line.transport {
    display: grid;
    grid-template-columns: 1fr 1fr 1fr;
}

.line label {
    display: flex;
    flex-direction: column;
    width: 100%;
}

label.inline {
    display: flex;
    flex-direction: row;
    justify-content: left;
    align-items: center;
    max-width: fit-content;
    gap: 0.5rem;
}

.membership {
    font-weight: 600;
    margin-top: 1rem;
}

label.check:hover,
label.check input:hover  {
    cursor: pointer;
}

textarea {
    max-width: 100%;
}

.rate {
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
}

.select-list {
    max-height: 50vh;
}

.bonus {
    display: flex;
    flex-direction: column;
    gap: 1rem;
}

.other {
    display: flex;
    align-items: flex-start;
    flex-wrap: wrap;
    max-width: 100%;
    gap: 1rem;
}

.other label {
    display: flex;
    flex-direction: column;
    flex-wrap: nowrap;
}

.extra-button {
    max-height: 3rem;
}

.free {
    max-width: 25rem;
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
}

.free label{
    display: grid;
    grid-template-columns: 1fr 1fr;
    align-items: start;
}

.toggle-materiel {
    padding: 1rem;
    border-radius: 0.5rem;

    transition: all 0.2s;
}

.toggle-materiel:hover {
    cursor: pointer;
    background-color: var(--disabled);
    transition: all 0.2s;
}

ul {
    display: flex;
    flex-direction: column;
    list-style-type: none;
    margin: 0;
    padding: 0;
    padding-bottom: 2rem;
    overflow-y: auto;
    overflow-x: hidden;
}

li {
    display: grid;
    grid-template-columns: 3fr 1fr 1fr 1fr 1fr;
    padding: 0 0.5rem;
    margin: 0;
    gap: 1rem;
    border-bottom: 1px solid var(--border);
}

li.extra {
    grid-template-columns: 3fr 1fr 1fr;
    align-items: center;
}

li:not(.head):nth-child(even) {
    background-color: var(--background-alt);
}

.total {
    display: flex;
    justify-content: center;
    align-items: center;
    width: 60rem;
    max-width: 95%;
}

.submit {
    width: 100%;
    position: sticky;
    bottom: -0.5rem;
    margin: 0;
    display: flex;
    flex-direction: column;
    align-items: center;
    border-bottom: 0;
    border: 1px solid var(--border-accent);
    border-bottom: 0;
    background-color: var(--background);
}

.buttons {
    display: flex;
    gap: 1rem;
}
</style>
