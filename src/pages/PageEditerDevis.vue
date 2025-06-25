<script setup>
import { invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';
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

const isAdmin = ref(false);
listen('log_in_admin', (event) => {
    isAdmin.value = event.payload;
});

const store = useDevisStore();
const tempExtrafield = ref({
    name: '',
    price: ''
});

const formulas = ref(null);
const contextName = ref('');
const saveMassage = ref({ result: 'success', message: '' });
const loadError = ref('');
const confirm = ref(null);

function addExtrafield(){
    if(tempExtrafield.value.name === '' || tempExtrafield.value.price === '') return;

    store.extraItems.push(tempExtrafield.value);
    tempExtrafield.value = {name: '', price: ''};
}

function removeExtraField(item) {
    const index = store.extraItems.indexOf(item);
    if (index !== -1) {
        store.extraItems.splice(index, 1);
    }
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
    if(document.id <= 0) {
        newDevis();
        setContextName();
        return;
    }

    if(document.facture) {
        props.setDocument(document, false);
    }

    try {
        await store.loadDocument(document);
        loadError.value = '';
        setContextName();
    } catch (err) {
        console.error(err + " on loading document " + id);
        loadError.value = err;
    }
}

function cancelDevis() {
    loadDocument({id: store.devisInfos.id, facture: false});
}

async function deleteDevis() {
    if(!(store.devisInfos.id > 0)) return;

    await invoke("delete_devis", { devisId: store.devisInfos.id });
    confirm.value = null;
    newDevis();
}

function confirmDelete() {
    if(!(store.devisInfos.id > 0)) return;

    confirm.value = 'delete';
}

function confirmCancel() {
    confirm.value = null;
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

onMounted(async() => {
    isAdmin.value = await invoke('is_admin');
    formulas.value = await invoke('get_loc_formulas');
    saveMassage.value = null;

    loadDocument(props.document);
    if(store.isFacture) {
        props.setDocument({id: store.devisInfos.id, facture: true}, false);
    }
    
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
watch(() => store.devisInfos.duration, (newVal, oldVal) => {
    if (!newVal || newVal <= 0) return;

    store.selectedItems.forEach(item => {
        if(item.duration === lastValidDuration) {
            store.setItem(item, 'unset', newVal);
        }
    });

    lastValidDuration = newVal;
});
</script>

<template>
    <div class="content">
        <div v-if="confirm" class="confirm">
            <div class="pop-up">
                <p>Êtes-vous sûr de vouloir supprimer <span>{{ contextName }}</span> ?</p>

                <div class="confirm-buttons">
                    <button @click="deleteDevis" class="delete" >Supprimer</button>
                    <button @click="confirmCancel" class="cancel">Annuler</button>
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
            <button v-if="isAdmin" @click="confirmDelete" class="delete">Supprimer</button>
        </div>
        <h2>Informations générales</h2>
        <section class="infos">
            <div class="line tech">
                <label>Nom du devis :
                    <input v-model="store.devisInfos.name"/>
                </label>
                <label>Date de début :
                    <input v-model="store.devisInfos.date"/>
                </label>
                <label>Durée (Jours) :
                    <input type="number" v-model="store.devisInfos.duration" min="1"/>
                </label>
            </div>
        </section>
        <h2>Client</h2>
        <section class="infos">
            <div class="line">
                <label>Nom du client :
                    <input v-model="store.clientInfos.name" list="client-names" @input="onNameInput"/>
                    <datalist id="client-names">
                        <option v-for="name in uniqueClientNames" :key="name">{{ name }}</option>
                    </datalist>
                </label>
                <label>Nom de l'evenement :
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
        </section>
        <h2>Utilitaires</h2>
        <section class="base">
            <div class="line tech">
                <label>Techniciens :
                    <input type="number" v-model="store.utilitaries.techQty" min="0"/>
                </label>
                <div class="rate">
                    <label>Prix unitaire :
                        <input type="number" v-model="store.utilitaries.techRate" min="0"/>
                    </label>
                    <label class="inline">Par Heure ?
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
                    <input type="number" v-model="store.utilitaries.transportRate" min="0"/>
                </label>
                <label>Total : 
                    <span>{{ (store.utilitaries.transportKm * store.utilitaries.transportRate).toFixed(2) }} €</span>
                </label>
            </div>
            <div class="line">
                <label class="inline">Adhésion ?
                    <input type="checkbox" v-model="store.utilitaries.membership"/>
                </label>
            </div>
        </section>
        <h2>Materiel</h2>
        <section class="materiel">
            <ListeSelectionDevis class="select-list" />

            <div v-if="store.selectedItems.length > 0">
                <h3>Materiel selectionné : </h3>
                <ul >
                    <li v-for="item in store.selectedItems" :data-id="item.id">
                        <p>{{ item.nom }}</p>
                        <p>{{ item.contrib.toFixed(2) }} €</p>
                        <p>{{ item.quantity }}</p>
                        <p>{{ item.duration }}</p>
                        <p>{{ item.totalPrice.toFixed(2) }} €</p>
                    </li>
                </ul>
            </div>
            <div v-if="store.extraItems.length > 0">
                <h3>Extras :</h3>
                <ul v-if="store.extraItems.length > 0">
                    <li v-for="item in store.extraItems" :data-id="item.id">
                        <p>{{ item.name }}</p>
                        <p>{{ item.price.toFixed(2) }} €</p>
                        <button @click="removeExtraField(item)">Supprimer</button>
                    </li>
                </ul>
            </div>
        </section>
        <h2>Autre</h2>
        <section class="bonus">
            <div class="other">
                <label>Nom : 
                    <input v-model="tempExtrafield.name"/>
                </label>
                <label>Prix (€) : 
                    <input type="number" v-model="tempExtrafield.price"/>
                </label>
                <button @click="addExtrafield">Ajouter</button>
            </div>
            <div class="free">
                <label>Geste commercial (€) : 
                    <input type="number" v-model="store.utilitaries.discountEuro"/>
                </label>
            </div>
        </section>
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
    max-height: 5rem;
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

.title {
    border: 2px solid var(--warning);
}

.title.new {
    border: 2px solid var(--success);
}

.context {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin: 1rem;
}

section {
    padding: 1rem;
    margin-bottom: 1rem;
    border-bottom: 1px solid var(--border);
}

section.base {
    display: flex;
    flex-direction: column;
    gap: 1rem;
    
}

h3 {
    margin-top: 2rem;
}

.line {
    display: grid;
    grid-template-columns: 1fr 1fr 1fr;
    align-items: start;
    gap: 1rem;
    width: 100%;
}

.line label {
    display: flex;
    flex-direction: column;
}

label.inline {
    flex-direction: row;
    display: inline-flex;
    align-items: center;
    gap: 0.5rem;
}

.rate {
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
}

.select-list {
    max-height: 40vh;
}

.bonus {
    display: flex;
    flex-direction: column;
    gap: 1rem;
}

.other {
    display: flex;
    gap: 1rem;
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

li:not(.head):nth-child(even) {
    background-color: var(--background-alt);
}

.submit {
    position: sticky;
    bottom: -0.5rem;
    margin: 0;
    display: flex;
    flex-direction: column;
    align-items: center;
    border-bottom: 0;
    border: 1px solid var(--border-accent);
    border-bottom: 0;
    border-top-left-radius: 0.5rem;
    border-top-right-radius: 0.5rem;
    background-color: var(--background);
}

.buttons {
    display: flex;
    gap: 1rem;
}
</style>
