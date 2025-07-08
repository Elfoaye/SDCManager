<script setup>
import { invoke } from '@tauri-apps/api/core';
import { ref, computed } from 'vue';
import { useBreadcrumb } from '../composables/breadcrumb';

const { setDocument } = defineProps(['setDocument']);

const { setBreadcrumb } = useBreadcrumb();
setBreadcrumb([
    { label: 'Accueil', page: null },
    { label: 'Devis & Factures', page: 'devparcour' }
]);

const columns = [
    { label: 'Type', key: 'etat'},
    { label: 'ID', key: 'id' },
    { label: 'Nom', key: 'nom' },
    { label: 'Date', key: 'date' },
    { label: 'Client', key: 'client_nom' },
    { label: 'Evenement', key: 'evenement' },
    { label: '', key: '' }
]

const listContent = ref([]);
Promise.all([
    invoke('get_devis_summaries'),
    invoke('get_factures_summaries')
]).then(([devis, factures]) => {
    listContent.value = [...devis, ...factures];
});

const sortProperty = ref(null);
const sortAsc = ref(true);
const filterSearch = ref('');
const filterType = ref(null);

function setFilterType(type) {
    if(filterType.value === type) {
        filterType.value = null;
    } else {
        filterType.value = type;
    }
}

const filteredContent = computed(() => {
    const query = String(filterSearch.value).trim().toLowerCase();

    return listContent.value.filter(devis =>  {
        if (query && !(String(devis.nom).toLowerCase().includes(query) || 
            String(devis.id).toLowerCase().includes(query) ||
            String(devis.client_nom).toLowerCase().includes(query) ||
            String(devis.evenement).toLowerCase().includes(query) ||
            String(devis.etat).toLowerCase().includes(query))) // Search bar
            return false;

        if (filterType.value === 'devis' && devis.etat.includes('facture') ||
            filterType.value === 'facture' && devis.etat.includes('devis'))
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
</script>

<template>
    <div class="content">
        <div class="title">
            <h1>Parcourir les devis et factures</h1>
        </div>
        <div class="list">
            <div class="search">
                <input v-model="filterSearch" class="searchbar" type="text" placeholder="Chercher par ID, nom, client..."/>
                <div class="filter-type">
                <button :class="{ selected: filterType === 'devis' }" @click="setFilterType('devis')">
                    Devis
                </button>
                <button :class="{ selected: filterType === 'facture' }" @click="setFilterType('facture')">
                    Factures
                </button>
            </div>
            </div>

            <li class="head">
                <button 
                    v-for="col in columns"
                    :key="col.key"
                    @click="setSort(col.key)"
                >
                    {{ col.label }} 
                    <template v-if="index !== columns.length - 1">
                        <span v-if="sortProperty === col.key">{{ sortAsc ? '▲' : '▼' }}</span>
                        <span v-else-if="sortProperty === null">
                            <svg width="16" height="16" viewBox="0 0 24 24" fill="currentColor">
                                <text x="50%" y="14" text-anchor="middle" font-size="10">▲</text>
                                <text x="50%" y="24" text-anchor="middle" font-size="10">▼</text>
                            </svg>
                        </span>
                    </template>
                </button>
            </li>
            <li class="new-item" @click="setDocument({id: 0, facture: false}, true)">+ Nouveau devis</li>
            <ul>
                <li v-for="item in sortedContent" @click="setDocument({id: item.id, facture: (item.etat?.includes('facture'))}, false)" :data-id="item.id">
                    <p class="state">{{ item.etat }}</p>
                    <p>{{ item.id }}</p>
                    <p>{{ item.nom }}</p>
                    <p>{{ item.date }}</p>
                    <p>{{ item.client_nom }}</p>
                    <p>{{ item.evenement }}</p>
                    <button v-if="!item.etat?.includes('facture')" class="modif" @click.stop="setDocument({id: item.id, facture: false}, true)">&#9998;</button>
                </li>
            </ul>
        </div>
    </div>
</template>

<style scoped>
.content {
    overflow-y: hidden;
}

.list {
    flex: 1;
    display: flex;
    flex-direction: column;
    min-height: 0;
}

.search {
    display: flex;
    align-items: center;
}

.searchbar {
    width: 50%;
    padding: 1rem;
}

.filter-type {
    max-width: 90%;
    justify-self: center;
    display: flex;
    margin: 0.5rem;
    padding: 0.5rem;
    gap: 1rem;
    border: 1px solid var(--border);
    border-radius: 0.5rem;
}

.filter-type button {
    min-width: 0;
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
    margin: 0;
}

ul {
    flex: 1;
    margin: 0;
    padding: 0;
    padding-bottom: 2rem;
    list-style-type: none;
    overflow-y: auto;
    overflow-x: hidden;
}

li, li.head {
    display: grid;
    grid-template-columns: 2fr 2fr 3fr 2fr 2fr 3fr 1fr;
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

li:not(.head) button {
    display: flex;
    align-items: center;
    justify-content: center;
    justify-self: center;
    align-self: center;
    max-width: 3rem;
    max-height: 3rem;
    font-size: 1.5rem;
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

li.new-item {
    grid-template-columns: 1fr;
    width: 100% - 1rem;
    padding: 1rem;
    padding-right: 0;
    font-weight: 600;
    background-color: var(--success-background) !important;
}

li.new-item:hover {
    background-color: var(--success) !important;
}

li p.state {
    text-transform: capitalize;
}
</style>