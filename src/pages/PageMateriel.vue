<script setup>
import { ref } from 'vue';
import ListeMateriel from '../components/ListeMateriel.vue';
import DisplayMateriel from '../components/DisplayMateriel.vue'

const display = ref(null)
function setDisplay(value) {
    display.value = value;
    console.log(value);
}
</script>

<template>
    <DisplayMateriel v-if="display !== null" :item="display" :setItem="setDisplay"/>
    <ListeMateriel v-else @display="setDisplay"/>
</template>

<style scoped>
</style>