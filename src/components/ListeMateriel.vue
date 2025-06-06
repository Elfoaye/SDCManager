<script setup>
import { invoke } from '@tauri-apps/api/core';
import { useBreadcrumb } from '../composables/breadcrumb';
import { ref, computed, watch } from 'vue';
import Multiselect from 'vue-multiselect';

const props = defineProps(['item', 'setItem', 'setCreate', 'modif']);

const { setBreadcrumb } = useBreadcrumb();

const columns = [
  { label: 'Nom', key: 'nom' },
  { label: 'Catégorie', key: 'item_type' },
  { label: 'Disponible', key: 'dispo' },
  { label: 'Total', key: 'total' },
  { label: 'Contrib/Jour', key: 'contrib' },
  { label: 'Valeur', key: 'valeur' }
]

const types = ref([]);
invoke('get_materiel_types').then((data) => types.value = data);

const listContent = ref([]);
const showFilters = ref(false);

// List sorting
const sortProperty = ref(null);
const sortAsc = ref(true);
//List filters
const filterSearch = ref('');
const filterType = ref([]);
const filterBorrow = ref('');
const filterDispo = ref('');

const filteredContent = computed(() => {
    const query = String(filterSearch.value).trim().toLowerCase();

    return listContent.value.filter(item =>  {
        if ((filterType.value.length > 0 && !filterType.value.includes(item.item_type)) || // Filters
            (filterBorrow.value === 'borrowed' && item.dispo === item.total || filterBorrow.value === 'available' && item.dispo < item.total) || // Borrowed
            (filterDispo.value === 'notdispo' && item.dispo > 0 || filterDispo.value === 'dispo' && item.dispo === 0) || // Dispo
            (query && !(String(item.nom).toLowerCase().includes(query) || String(item.item_type).toLowerCase().includes(query)))) // Search bar
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

function resetFilters() {
    filterSearch.value = '';
    filterType.value = [];
    filterBorrow.value = '';
    filterDispo.value = '';
}

async function updateData() {
    listContent.value = await invoke('get_materiel_data');
}
updateData();

async function updateItem(id) {
    const index = listContent.value.findIndex((item) => item.id === id);
    listContent.value[index] = await invoke('get_item_data', {id: id});
    notifyChange(index);
    return listContent.value[index];
}

function notifyChange(id) {
    const li = document.querySelector(`li[data-id="${id + 1}"]`);
    if (li) {
        li.classList.add('flash');
        void li.offsetWidth;
        li.classList.remove('flash');
        li.classList.add('transitioning');

        setTimeout(() => {
            li.classList.remove('transitioning');
        }, 1000); 
    }
}

function isSelected(itemToCheck) {
    return props.item && props.item.id === itemToCheck.id;
}

function toggleFilters() {
    showFilters.value = !showFilters.value;
}

function setSort(key) {
    if(sortProperty.value == key) {
        sortAsc.value = !sortAsc.value;
        return;
    }

    sortAsc.value = false;
    sortProperty.value = key;
}

function setFilterBorrow(value) {
    if(filterBorrow.value === value) {
        filterBorrow.value = null;
    } else {
        filterBorrow.value = value;
    }
}

function setFilterDispo(value) {
    if(filterDispo.value === value) {
        filterDispo.value = null;
    } else {
        filterDispo.value = value;
    }
}

defineExpose({ updateData, updateItem, listContent });

watch(() => props.modif, () => {
  setBreadcrumb([
    { label: 'Accueil', page: null },
    { label: 'Matériel', page: 'consult' },
    { label: props.modif ? 'Modifier' : 'Consulter', page: props.modif ? 'modif' : 'consult' }
  ]);
}, { immediate: true });
</script>

<template>
    <div class="content">
        <div class="title" :class="{modify: modif}">
            <h1 v-if="modif">Modifier le materiel</h1>
            <h1 v-else>Consulter le materiel</h1>
        </div>
        <div class="search">
            <input v-model="filterSearch" class="searchbar" name="searchbar" type="text" placeholder="Chercher par nom, catégorie..."/>
            <button @click="toggleFilters" :class="{ selected: showFilters }">Filtrer</button>
        </div>
        
        <div class="filters" v-if="showFilters">
            <h2>Filtres</h2>
            <button class="reset-filters" @click="resetFilters">Réinitialiser</button>

            <section class="type">
                <label for="type">Types</label>
                <Multiselect 
                    name="type" 
                    v-model="filterType" 
                    :options="types" 
                    :multiple="true" 
                    :close-on-select="false" 
                    placeholder="Selectionner des types"
                    selectLabel="Ajouter"
                    selectedLabel="Sélectionné"
                    deselectLabel="Retirer">
                </Multiselect>
            </section>
            
            <div class="filter-borrow">
                <button :class="{ selected: filterBorrow === 'borrowed' }" @click="setFilterBorrow('borrowed')">
                    Emprunté
                </button>
                <button :class="{ selected: filterBorrow === 'available' }" @click="setFilterBorrow('available')">
                    Non emprunté
                </button>
            </div>
            <div class="filter-dispo">
                <button :class="{ selected: filterDispo === 'dispo' }" @click="setFilterDispo('dispo')">
                    Disponible
                </button>
                <button :class="{ selected: filterDispo === 'notdispo' }" @click="setFilterDispo('notdispo')">
                    Non disponible
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
                <span v-if="sortProperty === col.key">{{ sortAsc ? '▲' : '▼' }}</span>
                
            </button>
        </li>
        <ul>
            <li v-if="modif" class="new-item" @click="setCreate">+ Ajouter un objet</li>
            <li v-for="item in sortedContent" @click="setItem(item)" :class="{ selected : isSelected(item) }" :data-id="item.id">
                <p>{{ item.nom }}</p>
                <p>{{ item.item_type }}</p>
                <p>{{ item.dispo }}</p>
                <p>{{ item.total }}</p>
                <p>{{ item.contrib.toFixed(2) }} €</p>
                <p>{{ item.valeur.toFixed(2) }} €</p>
            </li>
        </ul>
    </div>
</template>

<style src="vue-multiselect/dist/vue-multiselect.min.css"></style>

<style scoped>
.title.modify {
    background-color: var(--warning);
}

.search {
    width: 100%;
    margin-bottom: 0.5rem;
}

.searchbar {
    width: 50%;
    padding: 1rem;
    border-top-right-radius: 0;
    border-bottom-right-radius: 0;
}

button {
    margin-right: 1rem;
}

.search button {
    padding: 0.5rem;
    width: 4rem;
    border-left: 0;
    border-top-right-radius: 0.3rem;
    border-bottom-right-radius: 0.3rem;
    border-top-left-radius: 0;
    border-bottom-left-radius: 0;

    transition: all 0.2s;
}

.filters h2 {
    grid-column: span 3;
    margin: 0;
    padding-left: 0.5rem;
}

.reset-filters {
    grid-column: span 3;
    width: fit-content;
    padding: 0.5rem;
    margin-top: 0.5rem;
    margin-left: auto;
    margin-right: 0.5rem;
}

.type {
    grid-column: span 6;
}
.filter-borrow,
.filter-dispo {
    grid-column: span 3;
    max-width: 90%;
    justify-self: center;
    display: flex;
    margin: 0.5rem;
    padding: 0.5rem;
    gap: 1rem;
    border: 1px solid var(--border);
    border-radius: 0.5rem;
}

.filter-borrow button,
.filter-dispo button {
    min-width: 0;
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
    margin: 0;
}

.filters {
    display: grid;
    grid-template-columns: repeat(6, 1fr);
    padding: 0.5rem;
    border: 1px solid var(--border);
    border-radius: 0.5rem;
}

.filters p {
    margin: 0;
}

.filters section {
    padding: 0.5rem;
    border: 0;
}

.filters section label {
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
}

:deep(.multiselect__tags) {
    background: var(--background-alt);
    color: var(--text);
}

:deep(.multiselect__input) {
    background: var(--background-alt);
    color: var(--text);
}

:deep(.multiselect__input::placeholder) {
    color: var(--text-muted);
}

:deep(.multiselect__tag) {
    background: var(--accent-hover);
}

:deep(.multiselect__content) {
    background: var(--background-alt);
    color: var(--text);
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
    grid-template-columns: 3fr 1fr 1fr 1fr 1fr 1fr;
    padding: 0 0.5rem;
    margin: 0;
    gap: 1rem;
    border-bottom: 1px solid var(--border);
}

li.flash {
    background-color: var(--success) !important;
}

li.transitioning {
    transition: background-color 1s ease;
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
li p:nth-child(2) {
    padding-left: 0.4rem;
}
li p:nth-child(3),
li p:nth-child(4) {
    padding-left: 0.6rem;
}
li p:nth-child(5),
li p:nth-child(6) {
    padding-left: 0.8rem;
}

li.head {
    margin-top: 2rem;
    border-bottom: 1px solid var(--border-accent);
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
    width: 100%;
    padding: 1rem;
    font-weight: 600;
    background-color: var(--success-background);
}

li.new-item:hover {
    background-color: var(--success);
}
</style>