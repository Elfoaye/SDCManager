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

invoke('get_data').then((data) => console.log(data));

const list_content = [
    {nom:"Stairville Retro Flat Par 18x10W RGBWA UV",  type:"Lumière", disponible:8, total:12, contribution:9, valeur:269},
    {nom:"JBL EON710",  type:"Diff", disponible:2, total:2, contribution:15, valeur:535}, 
    {nom:"Shure SM58 LC ",  type:"Micros", disponible:6, total:16, contribution:4, valeur:109},
    {nom:"Stairville Retro Flat Par 18x10W RGBWA UV",  type:"Lumière", disponible:8, total:12, contribution:9, valeur:269},
    {nom:"JBL EON710",  type:"Diff", disponible:2, total:2, contribution:15, valeur:535}, 
    {nom:"Shure SM58 LC ",  type:"Micros", disponible:6, total:16, contribution:4, valeur:109},
    {nom:"Stairville Retro Flat Par 18x10W RGBWA UV",  type:"Lumière", disponible:8, total:12, contribution:9, valeur:269},
    {nom:"JBL EON710",  type:"Diff", disponible:2, total:2, contribution:15, valeur:535}, 
    {nom:"Shure SM58 LC ",  type:"Micros", disponible:6, total:16, contribution:4, valeur:109},
    {nom:"Stairville Retro Flat Par 18x10W RGBWA UV",  type:"Lumière", disponible:8, total:12, contribution:9, valeur:269},
    {nom:"JBL EON710",  type:"Diff", disponible:2, total:2, contribution:15, valeur:535}, 
    {nom:"Shure SM58 LC ",  type:"Micros", disponible:6, total:16, contribution:4, valeur:109},
    {nom:"Stairville Retro Flat Par 18x10W RGBWA UV",  type:"Lumière", disponible:8, total:12, contribution:9, valeur:269},
    {nom:"JBL EON710",  type:"Diff", disponible:2, total:2, contribution:15, valeur:535}, 
    {nom:"Shure SM58 LC ",  type:"Micros", disponible:6, total:16, contribution:4, valeur:109},
    {nom:"Stairville Retro Flat Par 18x10W RGBWA UV",  type:"Lumière", disponible:8, total:12, contribution:9, valeur:269},
    {nom:"JBL EON710",  type:"Diff", disponible:2, total:2, contribution:15, valeur:535}, 
    {nom:"Shure SM58 LC ",  type:"Micros", disponible:6, total:16, contribution:4, valeur:109}];

const types = ['Lumière','Diff','Micros'];

const columns = [
  { label: 'Nom', key: 'nom' },
  { label: 'Catégorie', key: 'type' },
  { label: 'Disponible', key: 'disponible' },
  { label: 'Total', key: 'total' },
  { label: 'Prix/Journée', key: 'contribution' },
  { label: 'Valeur', key: 'valeur' }
]

const show_filters = ref(false);

function toggle_filters() {
    show_filters.value = !show_filters.value;
}

const sort_property = ref(null);
const sort_asc = ref(true);

function set_sort(key) {
    if(sort_property.value == key) {
        sort_asc.value = !sort_asc.value;
        return;
    }

    sort_property.value = key;
}

const filter_search = ref('');
const filter_type = ref([]);
const filter_min_disp = ref('');
const filter_min_total = ref('');
const filter_max_price = ref('');

const filtered_content = computed(() => {
    const query = String(filter_search.value).trim().toLowerCase();

    return list_content.filter(item =>  {
        if (query && !(item.nom.toLowerCase().includes(query) || item.type.toLowerCase().includes(query)))
            return false;

        if (filter_type.value.length > 0 && !filter_type.value.includes(item.type))
            return false;

        if (filter_min_disp.value && filter_min_disp.value > item.disponible)
            return false;

        if (filter_min_disp.value && filter_min_total.value > item.total)
            return false;

        if (filter_max_price.value && filter_max_price.value < item.contribution)
            return false;

        return true;
    });
});

const sorted_content = computed(() => {
    if(!sort_property.value) return filtered_content.value;

    return [...filtered_content.value].sort((a, b) => {
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
            <input v-model="filter_search" type="text" placeholder="Chercher par nom, catégorie..."/>
            <button @click="toggle_filters">Filtrer</button>
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
                @click="set_sort(col.key)"
            >
                {{ col.label }} <span v-if="sort_property === col.key">{{ sort_asc ? '▲' : '▼' }}</span>
            </button>
        </li>
        <ul>
            <li v-for="item in sorted_content">
                <p>{{ item.nom }}</p>
                <p>{{ item.type }}</p>
                <p>{{ item.disponible }}</p>
                <p>{{ item.total }}</p>
                <p>{{ item.contribution }}</p>
                <p>{{ item.valeur }}</p>
            </li>
        </ul>
    </div>
</template>

<style src="vue-multiselect/dist/vue-multiselect.min.css">

</style>

<style scoped>
.content {
    flex-grow: 1;
    display: flex;
    flex-direction: column;
    max-width: 60rem;
    max-height: 100%;
    padding: 2rem;
    overflow: hidden;
}

input {
    padding: 0.5rem;
    border: 1px solid var(--border);
    border-radius: 0.3rem;
}

.search {
    margin-bottom: 2rem;
}

.search input {
    width: 40%;
    border-top-right-radius: 0;
    border-bottom-right-radius: 0;
}

.search button {
    height: 100%;
    padding: 0.5rem;
    width: 4rem;
    background-color: var(--accent);
    border: 1px solid var(--border);
    border-left: 0;
    border-top-right-radius: 0.3rem;
    border-bottom-right-radius: 0.3rem;

    transition: all 0.2s;
}

.search button:hover {
    background-color: var(--accent-hover);
    
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

:deep(.multiselect__tag) {
    background: var(--accent-hover);
}

ul {
    display: flex;
    flex-direction: column;
    list-style-type: none;
    margin: 0;
    padding: 0;
    overflow-y: auto;
}

li {
    display: grid;
    grid-template-columns: 3fr 1fr 1fr 1fr 1fr 1fr;
    padding: 0 0.5rem;
    margin: 0;
    gap: 1rem;
    align-items: center;
    border-bottom: 1px solid var(--border);
}

li:not(.head):nth-child(even) {
  background-color: var(--background-alt);
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
    background-color: var(--background);
    text-align: start;
    font-weight: 600;
}

.head button span {
  display: inline;
  margin-left: 0.25rem;
}
</style>