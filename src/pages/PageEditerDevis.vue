<script setup>
import { invoke } from '@tauri-apps/api/core';
import { ref, computed } from 'vue';
import { useBreadcrumb } from '../composables/breadcrumb';
import ListeSelectionDevis from '../components/ListeSelectionDevis.vue';

const { setBreadcrumb } = useBreadcrumb();
setBreadcrumb([
    { label: 'Accueil', page: null },
    { label: 'Devis & Factures', page: 'devparcour' },
    { label: 'Modifier', page: 'devmodif' }
]);

const formulas = ref(null);
invoke('get_loc_formulas').then((data) => formulas.value = data);

const selectedItems = ref([]);

const followingRate = computed(() => {
    if(!props.item || !formulas.value) return 0;
    return props.item.contrib * formulas.value.contrib_following;
});

function totalCost(item) {
    if(!item.duration.value || !item.quantity.value|| duration.value === 0) return 0;

    return item.quantity * (quantity.value * (props.item.contrib + (duration.value - 1) * followingRate.value));
};
</script>

<template>
    <div class="content">
        <div class="title">
            <h1>Editer le devis</h1>
        </div>
        <section class="base">
            <div class="tech">
                <label>Techniciens :
                    <input type="number" />
                </label>
                <label>Par Heure ?
                    <input type="checkbox" />
                </label>
                <label>Prix unitaire :
                    <input type="number" />
                </label>
                <label>Total :
                    <input type="number" />
                </label>
            </div>
            <div class="transport">
                <label>Transport (km):
                    <input type="number" />
                </label>
                <label>Prix unitaire :
                    <input type="number" />
                </label>
                <label>Total :
                    <input type="number" />
                </label>
            </div>
            <label>Adhésion ?
                <input type="checkbox" />
            </label>
        </section>
        <section class="materiel">
            <ListeSelectionDevis class="select-list" />
            <ul>
                <li v-for="item in selectedItems" :data-id="item.id">
                    <p>{{ item.nom }}</p>
                    <p>{{ item.contrib.toFixed(2) }} €</p>
                    <input v-model="item.qantity" />
                    <input v-model="item.duration" />
                    <p>{{ totalCost(item) }}</p>
                </li>
            </ul>
        </section>
        <section class="bonus">
            <div class="other">
                <label>Autre:
                    <input type="text" />
                </label>
                <label>Prix :
                    <input type="number" />
                </label>
            </div>
            <label>Geste commercial(%) :
                <input type="number" />
            </label>
            <label>Geste commercial(€) :
                <input type="number" />
            </label>
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
section {
    padding: 1rem;
    margin-bottom: 1rem;
    border-bottom: 1px solid var(--border);
}

section.base {
    display: flex;
    flex-direction: column;
    gap: 0.5rem
}

section.base div {
    display: grid;
    grid-template-columns: 1fr 1fr 1fr 1fr;
    gap: 1rem
}

section.base div label {
    display: flex;
    flex-direction: column;
}

.select-list {
    max-height: 40vh;
}
</style>
