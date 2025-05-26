<script setup>
defineProps(['item','setItem']);
</script>

<template>
    <button @click="setItem(null)">Back</button>
    <h1>{{ item.nom }}</h1>
    <h2>{{ item.item_type }}</h2>
    <p>{{ item.dispo }}</p>
    <p>{{ item.total }}</p>
    <p>{{ item.contrib }}</p>
    <p>{{ item.value }}</p>
</template>

<style>
</style>