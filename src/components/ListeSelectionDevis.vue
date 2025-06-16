<script setup>
import { invoke } from '@tauri-apps/api/core';
import { ref, computed, onMounted } from 'vue';
import { useDevisStore } from '../composables/devisStore'

const store = useDevisStore();

const listContent = ref([]);
const types = ref([]);
const formulas = ref(null);

const columns = [
  { label: 'Nom', key: 'nom' },
  { label: 'Catégorie', key: 'item_type' },
  { label: 'Total', key: 'total' },
  { label: 'Contrib/Jour', key: 'contrib' },
  { label: 'Quantité', key: 'quantit' },
  { label: 'Contrib/Total', key: 'total' },
]

const sortProperty = ref(null);
const sortAsc = ref(true);
const filterSearch = ref('');

const filteredContent = computed(() => {
    const query = String(filterSearch.value).trim().toLowerCase();

    return listContent.value.filter(item =>  {
        if (query && !(String(item.nom).toLowerCase().includes(query) || String(item.item_type).toLowerCase().includes(query))) // Search bar
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

function getQuantity(item) {
    const val = store.selectedItems.find(i => i.id === item.id)?.quantity ?? 0;
    return val === 0 ? '' : val;
}

function setSort(key) {
    if(sortProperty.value == key) {
        sortAsc.value = !sortAsc.value;
        return;
    }

    sortAsc.value = false;
    sortProperty.value = key;
}

function handleQuantityInput(item, event) {
    const value = parseInt(event.target.value, 10);
    if (!isNaN(value)) {
        store.setItemQuantity(item, value);
    }
}

const priceLoc = (item) => computed(() => {
    if(!item || !formulas.value) return 0;

    const quantity = store.selectedItems.find(i => i.id === item.id)?.quantity ?? 0; //TODO trouver quantity
    
    if(quantity <= 0 || store.devisInfos.duration === 0 ) return 0;

    return quantity * (item.contrib + (store.devisInfos.duration - 1) * formulas.value.followingRate);
});

onMounted(() => {
    invoke('get_materiel_types').then((data) => types.value = data);
    invoke('get_materiel_data').then((data) => listContent.value = data);
    invoke('get_loc_formulas').then((data) => formulas.value = data);
});
</script>

<template>
    <div class="content">
        <div class="search">
            <input v-model="filterSearch" class="searchbar" type="text" placeholder="Chercher par nom, catégorie..."/>
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
            <li v-for="item in sortedContent" :data-id="item.id">
                <p>{{ item.nom }}</p>
                <p>{{ item.item_type }}</p>
                <p>{{ item.total }}</p>
                <p>{{ item.contrib.toFixed(2) }} €</p>
                <input type="number" @input="handleQuantityInput(item, $event)" min="0" :max="item.total" :value="getQuantity(item)"/>
                <p v-if="priceLoc(item).value > 0">{{ priceLoc(item).value }}</p>
            </li>
        </ul>
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

li input {
    width: 4vw;
    max-width: 4rem;
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