<script setup>
import { invoke } from '@tauri-apps/api/core';
import { useBreadcrumb } from '../composables/breadcrumb';
import { ref, computed } from 'vue';
import Multiselect from 'vue-multiselect';

const { setBreadcrumb } = useBreadcrumb();
setBreadcrumb([
    { label: 'Accueil', page: null },
    { label: 'Matériel', page: 'mat' },
])

const props = defineProps(['item','setItem']);

const list_content = ref([]);
async function updateData() {
    list_content.value = await invoke('get_materiel_data');
}
updateData();
async function updateItem(id) {
    const index = list_content.value.findIndex((item) => item.id === id);
    list_content.value[index] = await invoke('get_item_data', {id: id});
    return list_content.value[index];
}

defineExpose({ updateData, updateItem, list_content });

function setDiplay(item) {
    props.setItem(item);
}

function isSelected(itemToCheck) {
    return props.item && props.item.id === itemToCheck.id;
}

const types = ref([]);
invoke('get_materiel_types').then((data) => types.value = data);

const columns = [
  { label: 'Nom', key: 'nom' },
  { label: 'Catégorie', key: 'item_type' },
  { label: 'Disponible', key: 'dispo' },
  { label: 'Total', key: 'total' },
  { label: 'Prix/Journée', key: 'contrib' },
  { label: 'Valeur', key: 'value' }
]

const show_filters = ref(false);

function toggleFilters() {
    show_filters.value = !show_filters.value;
}

const sort_property = ref(null);
const sort_asc = ref(true);

function setSort(key) {
    if(sort_property.value == key) {
        sort_asc.value = !sort_asc.value;
        return;
    }

    sort_asc.value = true;
    sort_property.value = key;
}

const filter_search = ref('');
const filter_type = ref([]);
const filter_min_disp = ref('');
const filter_min_total = ref('');
const filter_max_price = ref('');

const filteredContent = computed(() => {
    const query = String(filter_search.value).trim().toLowerCase();

    return list_content.value.filter(item =>  {
        if (query && !(String(item.nom).toLowerCase().includes(query) || String(item.item_type).toLowerCase().includes(query)))
            return false;

        if (filter_type.value.length > 0 && !filter_type.value.includes(item.item_type))
            return false;

        if (filter_min_disp.value && filter_min_disp.value > item.dispo)
            return false;

        if (filter_min_total.value && filter_min_total.value > item.total)
            return false;

        if (filter_max_price.value && filter_max_price.value < item.contrib)
            return false;

        return true;
    });
});

const sortedContent = computed(() => {
    if(!sort_property.value) return filteredContent.value;

    return [...filteredContent.value].sort((a, b) => {
        console.log(sort_property.value);
        const valA = a[sort_property.value] ;
        const valB = b[sort_property.value];
    
        if(typeof valA === 'number' && typeof valB === 'number') {
            return sort_asc.value ? valA - valB : valB - valA;
        } else {
            return sort_asc.value ? String(valA).localeCompare(String(valB)) :
                                    String(valB).localeCompare(String(valA));
        }
    });
});
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
                <label for="mindispo">Minimum disponible</label>
                <input v-model="filter_min_disp" label="mindispo" type="number" min="0"/>
            </section>
            <section class="mintotal">
                <label for="mintotal">Minimum total</label>
                <input v-model="filter_min_total" label="mintotal" type="number" min="0"/>
            </section class="number">
            <section class="prixmax">
                <label for="prixmax">Prix journalier maximum</label>
                <input v-model="filter_max_price" label="prixmax" type="number" min="0"/>
            </section>
        </div>

        <li class="head">
            <button 
                v-for="col in columns"
                :key="col.key"
                @click="setSort(col.key)"
            >
                {{ col.label }} <span v-if="sort_property === col.key">{{ sort_asc ? '▲' : '▼' }}</span>
            </button>
        </li>
        <ul>
            <li v-for="item in sortedContent" @click="setDiplay(item)" :class="{ selected : isSelected(item)}">
                <p>{{ item.nom }}</p>
                <p>{{ item.item_type }}</p>
                <p>{{ item.dispo }}</p>
                <p>{{ item.total }}</p>
                <p>{{ item.contrib }} €</p>
                <p>{{ item.value }} €</p>
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
    grid-area: title;
    margin: 0;
    padding-left: 0.5rem;
}

.type {
    grid-area: type;
}

.mindispo {
    grid-area: dispo;
}

.mintotal {
    grid-area: total;
}

.prixmax {
    grid-area: prix;
}

.filters {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(8rem, 1fr));
    grid-template-rows: auto;
    grid-template-areas: 
        "title title title" 
        "type type type"
        "dispo total prix";
    margin-bottom: 1rem;
    padding: 0.5rem;
    border: 1px solid var(--border);
    border-radius: 0.5rem;
}

.filters p {
    margin: 0;
}

.filters section {
    display: flex;
    flex-direction: column;
    padding: 0.5rem;
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

li {
    display: grid;
    grid-template-columns: 3fr 1fr 1fr 1fr 1fr 1fr;
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
}

li.head {
    border-bottom: 1px solid var(--border-accent);
}

.head button {
    display: inline-flex;
    border: none;
    cursor: pointer;
    padding-left: 0;
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