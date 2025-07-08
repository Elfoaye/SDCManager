<script setup>
import { invoke } from '@tauri-apps/api/core';
import { ref, computed } from 'vue';
import { useBreadcrumb } from '../composables/breadcrumb';

const { setBreadcrumb } = useBreadcrumb();
setBreadcrumb([
    { label: 'Accueil', page: null },
]);

const { setDocument, setPage } = defineProps(['setDocument', 'setPage']);

const devis = ref([]);
invoke('get_devis_summaries').then((data) => { devis.value = data });

const lastDevis = computed(() => devis.value.sort((a, b) => b.id - a.id).slice(0, 5));
const nextEvents = computed(() => {
    const today = new Date();

    return devis.value
        .filter(d => d.etat && d.etat.toLowerCase().includes('valide')
            && addDaysStr(d.date, d.durée) > today)
        .sort((a, b) => new Date(a.date) - new Date(b.date))
        .slice(0, 5);
});

const lastFactures = ref([]);
invoke('get_factures_summaries').then((data) => { lastFactures.value = data; });


function addDaysStr(dateStr, days) {
  const date = new Date(dateStr);
  date.setDate(date.getDate() + days);
  return date;
}

function formatDate(dateStr) {
    const date = new Date(dateStr);
    return new Intl.DateTimeFormat('fr-FR', {
        day: 'numeric',
        month: 'long',
        year: 'numeric'
    }).format(date);
}
</script>

<template>
    <div class="all">
        <h1>Bonjour !</h1>
        <div class="content-body">
            <section class="buttons">
                <button @click="setDocument({id: 0, facture: false}, true)" class="new">
                    <svg width="240" height="240" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
                        <path d="M19 13V15C19 17.8284 19 19.2426 18.1213 20.1213C17.2426 21 15.8284 21 13 21H11C8.17157 21 6.75736 21 5.87868 20.1213C5 19.2426 5 17.8284 5 15V9C5 6.17157 5 4.75736 5.87868 3.87868C6.75736 3 8.17157 3 11 3H12" stroke="currentColor" stroke-linecap="round"/>
                        <path d="M18 3L18 9" stroke="currentColor" stroke-linecap="round"/>
                        <path d="M21 6L15 6" stroke="currentColor" stroke-linecap="round"/>
                        <path d="M9 13L15 13" stroke="currentColor" stroke-linecap="round"/>
                        <path d="M9 9L13 9" stroke="currentColor" stroke-linecap="round"/>
                        <path d="M9 17L13 17" stroke="currentColor" stroke-linecap="round"/>
                    </svg>
                    Nouveau devis
                </button>
                <button @click="setPage('devparcour')">
                    <svg width="240" height="240" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
                        <path d="M9 7L13 7" stroke="currentColor" stroke-linecap="round"/>
                        <path d="M9 15L12 15" stroke="currentColor" stroke-linecap="round"/>
                        <path d="M9 11L15 11" stroke="currentColor" stroke-linecap="round"/>
                        <path d="M19 11V9C19 6.17157 19 4.75736 18.1213 3.87868C17.2426 3 15.8284 3 13 3H11C8.17157 3 6.75736 3 5.87868 3.87868C5 4.75736 5 6.17157 5 9V15C5 17.8284 5 19.2426 5.87868 20.1213C6.75736 21 8.17157 21 11 21H12" stroke="currentColor" stroke-linecap="round"/>
                        <circle cx="17.5" cy="17.5" r="2.5" stroke="currentColor"/>
                        <path d="M21 21L19.5 19.5" stroke="currentColor" stroke-linecap="round"/>
                    </svg>
                    Devis & Factures
                </button>
                <button @click="setPage('cal')">
                    <svg width="240" height="240" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
                        <path d="M17 3L17 7" stroke="currentColor" stroke-width="1.2" stroke-linecap="round"/>
                        <path d="M7 3L7 7" stroke="currentColor" stroke-width="1.2" stroke-linecap="round"/>
                        <path d="M3 10C3 8.11438 3 7.17157 3.58579 6.58579C4.17157 6 5.11438 6 7 6H17C18.8856 6 19.8284 6 20.4142 6.58579C21 7.17157 21 8.11438 21 10V11H3V10Z" stroke="currentColor" stroke-width="1.2"/>
                        <rect x="3" y="6" width="18" height="15" rx="2" stroke="currentColor" stroke-width="1.2"/>
                        <path d="M6 15H10" stroke="currentColor" stroke-opacity="0.25" stroke-width="1.2" stroke-linecap="round"/>
                        <path d="M14 15H18" stroke="currentColor" stroke-opacity="0.25" stroke-width="1.2" stroke-linecap="round"/>
                        <path d="M6 18H10" stroke="currentColor" stroke-opacity="0.25" stroke-width="1.2" stroke-linecap="round"/>
                        <path d="M14 18H18" stroke="currentColor" stroke-opacity="0.25" stroke-width="1.2" stroke-linecap="round"/>
                    </svg>
                    Calendrier
                </button>
                <button @click="setPage('consult')">
                    <svg width="240" height="240" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
                        <circle cx="11" cy="11" r="6" stroke="currentColor"/>
                        <path d="M11 8C10.606 8 10.2159 8.0776 9.85195 8.22836C9.48797 8.37913 9.15726 8.6001 8.87868 8.87868C8.6001 9.15726 8.37913 9.48797 8.22836 9.85195C8.0776 10.2159 8 10.606 8 11" stroke="currentColor" stroke-linecap="round"/>
                        <path d="M20 20L17 17" stroke="currentColor" stroke-linecap="round"/>
                    </svg>
                    Materiel
                </button>
            </section>
            <section class="quick-lists">
                <div class="list" v-if="lastFactures.length > 0">
                    <h2>Dernières factures: </h2>
                    <ul>
                        <li v-for="facture in lastFactures" @click="setDocument({ id: facture.id, facture: true })">
                            <p>{{ formatDate(facture.date) }}</p>
                            <h3>{{ facture.id + ' ' + facture.nom }}</h3>
                            <p>{{ facture.client_nom }}</p>
                        </li>
                    </ul>
                </div>
                <div class="list" v-if="lastDevis.length > 0">
                    <h2>Derniers devis: </h2>
                    <ul>
                        <li v-for="devis in lastDevis" @click="setDocument({ id: devis.id, facture: false })">
                            <h3>{{ devis.nom }}</h3>
                            <p>{{ devis.client_nom }}</p>
                        </li>
                    </ul>
                </div>
                <div class="list" v-if="nextEvents.length > 0">
                    <h2>Prochains évenements: </h2>
                    <ul>
                        <li v-for="devis in nextEvents" @click="setDocument({ id: devis.id, facture: false })">
                            <p>{{ formatDate(devis.date) }}</p>
                            <h3>{{ devis.nom }}</h3>
                            <p>{{ devis.client_nom }}</p>
                        </li>
                    </ul>
                </div>
            </section>
        </div>
    </div>
</template>

<style scoped>
.all {
    display: flex;
    flex-direction: column;
    justify-content: center;
    align-items: center;
    gap: 5vh;
    width: 100%;
    height: 100%;
}

.content-body {
    width: 100%;
    height: fit-content;
    display: flex;
    flex-direction: column;
    gap: 5vh;
    overflow: hidden;
    max-width: 80rem;
}

section {
    display: flex;
    flex-direction: row;
    justify-content: center;
    flex-wrap: wrap;
    gap: 1vw;
}

section.quick-lists {
    display: grid;
    grid-template-columns: 1fr 1fr 1fr;
    width: fit-content;
    max-width: 90%;
    align-self: center;
    column-gap: 2rem;
}

button {
    justify-self: center;
    display: flex;
    flex-direction: column;
    align-items: center;
    height: 8rem;
    width: 8rem;
    max-width: 100%;
    gap: 0.5rem;
}

.list {
    justify-self: center;
    width: 100%;
    display: flex;
    flex-direction: column;
    justify-content: center;
    height: fit-content;
    max-height: 40vh;
    overflow-x: hidden;
    gap: 1vh;
    padding: 0.5rem;
    border: 1px solid var(--border);
    border-radius: 0.5rem;
    background-color: var(--background-alt);
}

h2 {
    text-align: center;
}

ul {
    max-height: 100%;
    overflow-y: auto;
    overflow-x: hidden;
    list-style-type: none;
    margin: 0;
    padding: 0;
    width: 100%;
    text-align: center;
    
}

li {
    width: 100%;
    flex: 1;
    padding: 0.5rem;
    border: 1px solid var(--border);
    border-radius: 0.5rem;
    margin-bottom: 0.5rem;
    box-sizing: border-box;

    transition: all 0.2s;
}

li:hover {
    cursor: pointer;
    background-color: var(--surface-hover);

    transition: all 0.2s;
}

li h3 {
    margin: 0;
}

li p {
    margin: 0;
}
</style>