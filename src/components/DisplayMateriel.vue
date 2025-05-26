<script setup>
import { invoke } from '@tauri-apps/api/core';
import { ref } from 'vue';
defineProps(['item','setItem']);

const formulas = ref(null);
invoke('get_loc_formulas').then((data) => formulas.value = data);
</script>

<template>
    <div class="itemCard">
        <button @click="setItem(null)">Back</button>
        <h1>{{ item.nom }}</h1>
        <h2>{{ item.item_type }}</h2>
        <p>Disponible : {{ item.dispo }}</p>
        <p>Total : {{ item.total }}</p>
        <p>Contribution pour 1 jour : {{ item.contrib }}</p>
        <p>Contribution pour chaque jour suivant : {{ item.contrib * formulas.contrib_following }}</p>
        <p>Valeur de remplacement : {{ item.value }}</p>
        <p>Nombre de sorties : {{ item.nb_sorties }}</p>
    </div>
</template>

<style scopped>
.itemCard {
    height: auto;
    margin: 0.5rem;
    padding: 1em;
    border: 1px solid var(--border);
    border-radius: 0.5rem;
}
</style>