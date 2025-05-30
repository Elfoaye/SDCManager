<script setup>
import { invoke } from '@tauri-apps/api/core';
import { useBreadcrumb } from '../composables/breadcrumb';
import { ref, computed } from 'vue';
import Multiselect from 'vue-multiselect';

const props = defineProps(['item','setItem']);

const { setBreadcrumb } = useBreadcrumb();
setBreadcrumb([
    { label: 'Accueil', page: null },
    { label: 'Matériel', page: 'mat' },
])

const columns = [
  { label: 'Nom', key: 'nom' },
  { label: 'Catégorie', key: 'item_type' },
  { label: 'Disponible', key: 'dispo' },
  { label: 'Total', key: 'total' },
  { label: 'Contrib/Jour', key: 'contrib' },
  { label: 'Valeur', key: 'value' }
]

const types = ref([]);
invoke('get_materiel_types').then((data) => types.value = data);

const list_content = ref([]);
const show_filters = ref(false);

// List sorting
const sort_property = ref(null);
const sort_asc = ref(true);
//List filters
const filter_search = ref('');
const filter_type = ref([]);
const filter_min_disp = ref('');
const filter_min_total = ref('');
const filter_max_price = ref('');
const filter_borrow = ref('');
const filter_dispo = ref('');

const filteredContent = computed(() => {
    const query = String(filter_search.value).trim().toLowerCase();

    return list_content.value.filter(item =>  {
        if ((filter_type.value.length > 0 && !filter_type.value.includes(item.item_type)) || // Filters
            (filter_min_disp.value && filter_min_disp.value > item.dispo) || // Min dispo
            (filter_min_total.value && filter_min_total.value > item.total) || // Min total
            (filter_max_price.value && filter_max_price.value < item.contrib) || // Max contrib
            (filter_borrow.value === 'borrowed' && item.dispo === item.total || filter_borrow.value === 'available' && item.dispo < item.total) || // Borrowed
            (filter_dispo.value === 'notdispo' && item.dispo > 0 || filter_dispo.value === 'dispo' && item.dispo === 0) || // Dispo
            (query && !(String(item.nom).toLowerCase().includes(query) || String(item.item_type).toLowerCase().includes(query)))) // Search bar
            return false;

        return true;
    });
});

const sortedContent = computed(() => {
    if(!sort_property.value) return filteredContent.value;

    return [...filteredContent.value].sort((a, b) => {
        const valA = a[sort_property.value] ;
        const valB = b[sort_property.value];
    
        if(typeof valA === 'number' && typeof valB === 'number') {
            return sort_asc.value ? valA - valB : valB - valA;
        } else {
            return sort_asc.value ? String(valB).localeCompare(String(valA)) :
                                    String(valA).localeCompare(String(valB));
        }
    });
});

async function updateData() {
    list_content.value = await invoke('get_materiel_data');
}
updateData();

async function updateItem(id) {
    const index = list_content.value.findIndex((item) => item.id === id);
    list_content.value[index] = await invoke('get_item_data', {id: id});
    notifyChange(index);
    return list_content.value[index];
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
    show_filters.value = !show_filters.value;
}

function setSort(key) {
    if(sort_property.value == key) {
        sort_asc.value = !sort_asc.value;
        return;
    }

    sort_asc.value = false;
    sort_property.value = key;
}

function setFilterBorrow(value) {
    if(filter_borrow.value === value) {
        filter_borrow.value = null;
    } else {
        filter_borrow.value = value;
    }
}

function setFilterDispo(value) {
    if(filter_dispo.value === value) {
        filter_dispo.value = null;
    } else {
        filter_dispo.value = value;
    }
}

defineExpose({ updateData, updateItem, list_content });
</script>

<template>
    <div class="content">
        <div class="search">
            <input v-model="filter_search" class="searchbar" name="searchbar" type="text" placeholder="Chercher par nom, catégorie..."/>
            <button @click="toggleFilters">Filtrer</button>
        </div>
        
        <div class="filters" v-if="show_filters">
            <h2>Filtres</h2>
            <section class="type">
                <label for="type">Types</label>
                <Multiselect 
                    name="type" 
                    v-model="filter_type" 
                    :options="types" 
                    :multiple="true" 
                    :close-on-select="false" 
                    placeholder="Selectionner des types"
                    selectLabel="Ajouter"
                    selectedLabel="Sélectionné"
                    deselectLabel="Retirer">
                </Multiselect>
            </section>

            <section class="mindispo">
                <label>Minimum disponible</label>
                <input v-model="filter_min_disp" type="number" min="0" placeholder="Au moins ... disponibles"/>
            </section>
            <section class="mintotal">
                <label>Minimum total</label>
                <input v-model="filter_min_total" type="number" min="0" placeholder="Au moins ... au total"/>
            </section>
            <section class="prixmax">
                <label>Contribution maximum</label>
                <input v-model="filter_max_price" type="number" min="0" placeholder="Coûtant moins de ... par jour"/>
            </section>
            
            <div class="filter-borrow">
                <button :class="{ selected: filter_borrow === 'borrowed' }" @click="setFilterBorrow('borrowed')">
                    Emprunté
                </button>
                <button :class="{ selected: filter_borrow === 'available' }" @click="setFilterBorrow('available')">
                    Non emprunté
                </button>
            </div>
            <div class="filter-dispo">
                <button :class="{ selected: filter_dispo === 'dispo' }" @click="setFilterDispo('dispo')">
                    Disponible
                </button>
                <button :class="{ selected: filter_dispo === 'notdispo' }" @click="setFilterDispo('notdispo')">
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
                <span v-if="sort_property === col.key">{{ sort_asc ? '▲' : '▼' }}</span>
                
            </button>
        </li>
        <ul>
            <li v-for="item in sortedContent" @click="setItem(item)" :class="{ selected : isSelected(item) }" :data-id="item.id">
                <p>{{ item.nom }}</p>
                <p>{{ item.item_type }}</p>
                <p>{{ item.dispo }}</p>
                <p>{{ item.total }}</p>
                <p>{{ item.contrib.toFixed(2) }} €</p>
                <p>{{ item.value.toFixed(2) }} €</p>
            </li>
        </ul>
    </div>
</template>

<style src="vue-multiselect/dist/vue-multiselect.min.css"></style>

<style scoped>
.content {
    flex-grow: 2;
    display: flex;
    flex-direction: column;
    max-width: 60rem;
    max-height: 100%;
    margin: 0.5rem;
    overflow: hidden;
}

input {
    background-color: var(--background-alt);
    color: var(--text);
    border: 1px solid var(--border);
    border-radius: 0.3rem;
}

.search {
    width: 100%;
    margin-bottom: 2rem;
}

.searchbar {
    width: 100%;
    padding: 1rem;
    border-top-right-radius: 0;
    border-bottom-right-radius: 0;
}

.search button {
    height: 100%;
    padding: 0.5rem;
    width: 4rem;
    background-color: var(--accent);
    color: var(--text);
    border: 1px solid var(--border);
    border-left: 0;
    border-top-right-radius: 0.3rem;
    border-bottom-right-radius: 0.3rem;
    border-top-left-radius: 0;
    border-bottom-left-radius: 0;

    transition: all 0.2s;
}

.search button:hover {
    background-color: var(--accent-hover);
    cursor: pointer;
    
    transition: all 0.2s;
}

.filters h2 {
    grid-column: span 6;
    margin: 0;
    padding-left: 0.5rem;
}

.type {
    grid-column: span 6;
}
.mindispo,
.mintotal,
.prixmax {
    grid-column: span 2;
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
    margin-bottom: 1rem;
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
</style>