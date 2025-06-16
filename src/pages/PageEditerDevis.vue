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

const followingRate = computed(() => {
    if(!props.item || !formulas.value) return 0;
    return props.item.contrib * formulas.value.contrib_following;
});

function setExtrafield(){
    if(tempExtrafield.value.name === '' || tempExtrafield.value.price === '') return;

    store.extraItems.push(tempExtrafield.value);
    tempExtrafield.value = {name: '', price: ''};
}

function totalItemCost(item) {
    if(!formulas.value || !item.duration || !item.quantity|| duration === 0) return 0;

    return item.quantity * (quantity.value * (props.item.contrib 
    + (duration.value - 1) * formulas.value.followingRate.value));
};

function finalCost() {
    return 0;
}

function setTechRate() {
    if(!formulas.value) {
        console.error("No formulas imported yet");
        return;
    }
    console.log("Setting tech rate");
    store.techRate = store.techHourly ? formulas.tech_hour : formulas.tech_day;
}

onMounted( async() => {
    formulas.value = await invoke('get_loc_formulas');

    if(store.techRate === 0) {
        setTechRate();
    }
});

watch(() => store.techHourly, () => {
    setTechRate();
});
</script>

<template>
    <div class="content">
        <div class="title">
            <h1>Editer le devis</h1>
        </div>
        <h2>Informations générales</h2>
        <section class="infos">
            <div class="line tech">
                <label>Nom du devis :
                    <input v-model="store.devisInfos.name"/>
                </label>
                <label>Date :
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
                    <input v-model="store.clientInfos.name"/>
                </label>
                <label>Telephone du client :
                    <input v-model="store.clientInfos.eventName"/>
                </label>
            </div>
        </section>
        <h2>Utilitaires</h2>
        <section class="base">
            <div class="line tech">
                <label>Techniciens :
                    <input type="number" v-model="store.utilitaries.techQty"/>
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
                    <span>{{ store.utilitaries.techQty * store.utilitaries.techRate }}</span>
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
                    <span>{{ store.utilitaries.transportKm * store.utilitaries.transportRate }}</span>
                </label>
            </div>
            <div class="line adhesion">
                <label class="inline">Adhésion ?
                    <input type="checkbox" v-model="store.utilitaries.adhesion"/>
                </label>
            </div>
        </section>
        <h2>Materiel</h2>
        <section class="materiel">
            <ListeSelectionDevis class="select-list" />

            <div v-if="store.selectedItems.length > 0">
                <h3>Items selectionnés : </h3>
                <ul >
                    <li v-for="item in store.selectedItems" :data-id="item.id">
                        <p>{{ item.nom }}</p>
                        <p>{{ item.contrib.toFixed(2) }} €</p>
                        <input v-model="item.quantity" />
                        <p>{{ totalItemCost(item) }}</p>
                    </li>
                </ul>
            </div>
            <div v-if="store.extraItems.length > 0">
                <h3>Extras :</h3>
                <ul v-if="store.extraItems.length > 0">
                    <li v-for="item in store.extraItems" :data-id="item.id">
                        <p>{{ item.name }}</p>
                        <p>{{ item.price.toFixed(2) }} €</p>
                        <button>Supprimer</button>
                    </li>
                </ul>
            </div>
        </section>
        <h2>Autre</h2>
        <section class="bonus">
            <div class="other">
                <label>Nom:
                    <input v-model="tempExtrafield.name"/>
                </label>
                <label>Prix :
                    <input type="number" v-model="tempExtrafield.price"/>
                </label>
                <button @click="setExtrafield">Ajouter</button>
            </div>
            <div class="free">
                <label>Geste commercial(%) :
                    <input type="number" v-model="store.utilitaries.discountPercent"/>
                </label>
                <label>Geste commercial(€) :
                    <input type="number" v-model="store.utilitaries.discountEuro"/>
                </label>
            </div>
        </section>
        <section class="total">
            <h2><span>Prix total : {{ finalCost() }}</span></h2>
        </section>
        <!-- <section class="preview">

        </section> -->
        <section class="submit">
            <button>
                Sauvegarder
            </button>
            <button>
                Annuler
            </button>
            <button>
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
            </button>
        </section>
    </div>
</template>

<style scoped>
.content {
    max-height: 100%;
    overflow-y: auto;
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
    grid-template-columns: 3fr 1fr 1fr 1fr;
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
