<script setup>
import { ref, computed } from 'vue';
import Multiselect from 'vue-multiselect';

const list_content = [
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

const filter_search = ref('');
const filter_type = ref(null);

const filtered_content = computed(() => {
    const query = String(filter_search.value).trim().toLowerCase();

    return list_content.filter(item =>  {
        // Search bar
        if (query && !(item.nom.toLowerCase().includes(query) || item.type.toLowerCase().includes(query)))
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

        <p v-if="filter_search">Recherche actuelle : {{ filter_search }}</p>

        <div class="filters" v-if="show_filters">
            <p>Filtres</p>
            <Multiselect v-model="filter_type" :options="types" :multiple="true" :close-on-select="false"></Multiselect>
        </div>

        <ul>
            <li class = "head">
                <button 
                    v-for="col in columns"
                    :key="col.key"
                    @click="set_sort(col.key)"
                >
                {{ col.label }} <span v-if="sort_property === col.key">{{ sort_asc ? '▲' : '▼' }}</span>
            </button>
            </li>
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
    padding: 2rem;
}

.search {
    margin-bottom: 2rem;
}

input {
    padding: 0.5rem;
}

.search button {
    height: 100%;
    padding-left: 0.5rem;
    padding-right: 0.5rem;
}

.filters {
    margin-bottom: 1rem;
    padding: 0.5rem;
    border: 1px solid var(--border);
    border-radius: 0.5rem;
}

.filters p {
    margin: 0;
}

ul {
    display: flex;
    flex-direction: column;
    list-style-type: none;
    margin: 0;
    padding: 0;
}

li {
    display: grid;
    grid-template-columns: 3fr 1fr 1fr 1fr 1fr 1fr;
    padding: 0 0.5rem;
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