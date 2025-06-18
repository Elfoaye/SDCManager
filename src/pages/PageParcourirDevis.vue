<script setup>
import { invoke } from '@tauri-apps/api/core';
import { ref, computed } from 'vue';
import { useBreadcrumb } from '../composables/breadcrumb';

const props = defineProps(['setDevis']);

const { setBreadcrumb } = useBreadcrumb();
setBreadcrumb([
    { label: 'Accueil', page: null },
    { label: 'Devis & Factures', page: 'devparcour' }
]);

const columns = [
    { label: 'ID', key: 'id' },
    { label: 'Nom', key: 'nom' },
    { label: 'Date', key: 'date' },
    { label: 'Client', key: 'client_nom' },
    { label: 'Evenement', key: 'evenement' }
]

const listContent = ref([]);
invoke('get_devis_summaries').then((data) => listContent.value = data);

const sortProperty = ref(null);
const sortAsc = ref(true);
const filterSearch = ref('');

const filteredContent = computed(() => {
    const query = String(filterSearch.value).trim().toLowerCase();

    return listContent.value.filter(devis =>  {
        if (query && !(String(devis.nom).toLowerCase().includes(query) || 
            String(devis.id).toLowerCase().includes(query) ||
            String(devis.client_nom).toLowerCase().includes(query) ||
            String(devis.evenement).toLowerCase().includes(query))) // Search bar
            return false;

        return true;
    });
});

const sortedContent = computed(() => {
    if(!sortProperty.value) return filteredContent.value;

    return [...filteredContent.value].sort((a, b) => {
        const valA = a[sortProperty.value] ;
        const valB = b[sortProperty.value];
    
        if(typeof valA === 'number' && typeof valB === 'number') {
            return sortAsc.value ? valA - valB : valB - valA;
        } else {
            return sortAsc.value ? String(valB).localeCompare(String(valA)) :
                                    String(valA).localeCompare(String(valB));
        }
    });
});

function setSort(key) {
    if(sortProperty.value == key) {
        sortAsc.value = !sortAsc.value;
        return;
    }

    sortAsc.value = false;
    sortProperty.value = key;
}

function setDevis(id) {
    props.setDevis(id);
}
</script>

<template>
    <div class="content">
        <div class="title">
            <h1>Parcourir les devis et factures</h1>
        </div>
        <div class="list">
            <div class="search">
                <input v-model="filterSearch" class="searchbar" type="text" placeholder="Chercher par ID, nom, client..."/>
            </div>

            <li class="head">
                <button 
                    v-for="col in columns"
                    :key="col.key"
                    @click="setSort(col.key)"
                >
                    {{ col.label }} 
                    <span v-if="sortProperty === col.key">{{ sortAsc ? '▲' : '▼' }}</span>
                </button>
            </li>
            <ul>
                <li v-for="item in sortedContent" @click="setDevis(item.id)" :data-id="item.id">
                    <p>{{ item.id }}</p>
                    <p>{{ item.nom }}</p>
                    <p>{{ item.date }}</p>
                    <p>{{ item.client_nom }}</p>
                    <p>{{ item.evenement }}</p>
                </li>
            </ul>
        </div>
    </div>
</template>

<style scoped>
.searchbar {
    width: 50%;
    padding: 1rem;
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

li, li.head {
    display: grid;
    grid-template-columns: 1fr 2fr 1fr 1fr 1fr;
    padding: 0 0.5rem;
    margin: 0;
    gap: 1rem;
    border-bottom: 1px solid var(--border);
}

li:not(.head):nth-child(even) {
    background-color: var(--background-alt);
}

li:not(.head):hover {
    cursor: pointer;
    background-color: var(--surface-hover);
}

li:not(.head).selected,
li:not(.head).selected:hover {
    background-color: var(--surface-selected);
}

li p {
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
    padding-left: 0
}

li.head {
    margin-top: 2rem;
    border-bottom: 1px solid var(--border-accent);
}

li:not(.head):hover {
    cursor: pointer;
    background-color: var(--surface-hover);
}

.head button {
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
    display: block;
    width: 100%;
    border: 0;
    border-radius: 0;
    cursor: pointer;
    padding: 0;
    margin: 0;
    padding-bottom: 0.5rem;
    color: var(--text);
    background-color: var(--background);
    text-align: start;
    font-weight: 600;
}

.head button span {
  display: inline;
  margin-left: 0.25rem;
}
</style>