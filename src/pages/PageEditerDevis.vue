<script setup>
import { invoke } from '@tauri-apps/api/core';
import { ref, computed } from 'vue';
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

const formulas = ref(null);
invoke('get_loc_formulas').then((data) => formulas.value = data);

const byHour = ref(false);

const followingRate = computed(() => {
    if(!props.item || !formulas.value) return 0;
    return props.item.contrib * formulas.value.contrib_following;
});

function totalItemCost(item) {
    if(!item.duration.value || !item.quantity.value|| duration.value === 0) return 0;

    return item.quantity * (quantity.value * (props.item.contrib + (duration.value - 1) * followingRate.value));
};

function finalCost() {
    return 0;
}
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
                    <input v-model="store.extraFields.name"/>
                </label>
                <label>Date :
                    <input v-model="store.extraFields.date"/>
                </label>
                <div class="rate">
                    <label>Durée (Jours) :
                        <input type="number" v-model="store.extraFields.duration"/>
                    </label>
                    <label class="inline">Par Heure ?
                        <input type="checkbox" v-model="store.extraFields.hourly"/>
                    </label>
                </div>
                <label>Nom du client :
                    <input v-model="store.extraFields.clientName"/>
                </label>
                <label>Addresse du client :
                    <input v-model="store.extraFields.clientAdress"/>
                </label>
            </div>
        </section>
        <h2>Materiel</h2>
        <section class="materiel">
            <ListeSelectionDevis class="select-list" />
            <ul>
                <li v-for="item in selectedItems" :data-id="item.id">
                    <p>{{ item.nom }}</p>
                    <p>{{ item.contrib.toFixed(2) }} €</p>
                    <input v-model="item.qantity" />
                    <p>{{ totalItemCost(item) }}</p>
                </li>
            </ul>
        </section>
        <h2>Utilitaires</h2>
        <section class="base">
            <div class="line tech">
                <label>Techniciens :
                    <input type="number" v-model="store.extraFields.techQty"/>
                </label>
                <div class="rate">
                    <label>Prix unitaire :
                        <input type="number" v-model="store.extraFields.techRate"/>
                    </label>
                    <label class="inline">Par Heure ?
                        <input type="checkbox" v-model="store.extraFields.techHourly"/>
                    </label>
                </div>
                <span>Total : {{ store.extraFields.techQty * store.extraFields.techRate }}</span>
            </div>
            <div class="line transport">
                <label>Transport (km):
                    <input type="number" v-model="store.extraFields.transportKm"/>
                </label>
                <label>Prix unitaire :
                    <input type="number" v-model="store.extraFields.transportRate"/>
                </label>
                <span>Total : {{ store.extraFields.transportKm * store.extraFields.transportRate }}</span>
            </div>
            <div class="line adhesion">
                <label class="inline">Adhésion ?
                    <input type="checkbox" v-model="store.extraFields.adhesion"/>
                </label>
            </div>
        </section>
        <h2>Autre</h2>
        <section class="bonus">
            <div class="other">
                <label>Autre:
                    <input v-model="store.extraFields.otherLabel"/>
                </label>
                <label>Prix :
                    <input type="number" v-model="store.extraFields.otherPrice"/>
                </label>
            </div>
            <div class="free">
                <label>Geste commercial(%) :
                    <input type="number" v-model="store.extraFields.discountPercent"/>
                </label>
                <label>Geste commercial(€) :
                    <input type="number" v-model="store.extraFields.discountEuro"/>
                </label>
            </div>
        </section>
        <section class="total">
            <h2><span>Prix total : {{ finalCost() }}</span></h2>
        </section>
        <section class="preview">

        </section>
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

.submit {
    display: flex;
    gap: 1rem;
    border-bottom: 0;
}
</style>
