<script setup>
import { invoke } from '@tauri-apps/api/core';
import { ref, computed, onMounted, watch } from 'vue';
import { useBreadcrumb } from '../composables/breadcrumb';
import { useDevisStore } from '../composables/devisStore';
import ListeSelectionDevis from '../components/ListeSelectionDevis.vue';

const { setBreadcrumb } = useBreadcrumb();
setBreadcrumb([
    { label: 'Accueil', page: null },
    { label: 'Devis & Factures', page: 'devparcour' },
    { label: 'Modifier', page: 'devmodif' }
]);

const store = useDevisStore();
const tempExtrafield = ref({
    name: '',
    price: ''
});

const formulas = ref(null);

const saveMassage = ref(null);

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

function setTechRate() {
    store.utilitaries.techRate = store.utilitaries.techHourly ? formulas.value.tech_hour : formulas.value.tech_day;
}

function newDevis() {
    store.reset();
    setTechRate();
    store.utilitaries.transportRate = formulas.value.transport_km;
    saveMassage.value = null;
}

// TODO Faire marcher ça
function loadDevis(id) {
    console.log("Chargement du devis " + id);
    store.loadDevis(id);
    saveMassage.value = null;
}

async function saveDevis() {
    saveMassage.value = await store.saveDevis();
}

function cancelDevis() {
    if(store.devisInfos.id == 0) {
        newDevis();
    } else {
        loadDevis(store.devisInfos.id);
    }
}

onMounted(async() => {
    formulas.value = await invoke('get_loc_formulas');
    saveMassage.value = null;

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
</script>

<template>
    <div class="content">
        <div class="title">
            <h1>Editer le devis</h1>
        </div>
        <div class="context">
            <p v-if="store.devisInfos.id > 0">Edition du devis {{ store.devisInfos.id + " " + store.devisInfos.name }}</p>
            <p v-else>Nouveau devis</p>
            <button @click="newDevis">Nouveau devis</button>
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
                    <input type="number" v-model="store.devisInfos.duration"/>
                </label>
            </div>
        </section>
        <h2>Client</h2>
        <section class="infos">
            <div class="line">
                <label>Nom du client :
                    <input v-model="store.clientInfos.name"/>
                </label>
                <label>Nom de l'evenement :
                    <input v-model="store.clientInfos.eventName"/>
                </label>
                <label>Addresse du client :
                    <textarea v-model="store.clientInfos.adress"></textarea>
                </label>
            </div>
            <div class="line">
                <label>Mail du client :
                    <input v-model="store.clientInfos.phone"/>
                </label>
                <label>Telephone du client :
                    <input v-model="store.clientInfos.mail"/>
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
                        <input type="number" v-model="store.utilitaries.techRate"/>
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
                    <input type="number" v-model="store.utilitaries.transportKm"/>
                </label>
                <label>Prix unitaire :
                    <input type="number" v-model="store.utilitaries.transportRate"/>
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
        <!-- <section class="preview">

        </section> -->
        <section class="submit">
            <div class="buttons">
                <button @click="saveDevis">
                    {{ store.devisInfos.id > 0 ? 'Mettre à jour' : 'Enregistrer' }}
                </button>
                <button @click="cancelDevis">
                    Annuler
                </button>
                <!-- <button>
                    Apperçu
                </button>
                <button>
                    Télecharger
                </button>
                <button>
                    Facturer
                </button>
                <button>
                    Dupliquer
                </button> -->
            </div>
            <p v-if="saveMassage" :class="saveMassage.result">{{ saveMassage.message }}</p>
        </section>
    </div>
</template>

<style scoped>
.context {
    display: flex;
    justify-content: space-between;
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
    display: flex;
    gap: 1rem;
    border-bottom: 0;
}
</style>
