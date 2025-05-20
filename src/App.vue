<script setup>
import { ref } from 'vue';

import NavBar from './components/NavBar.vue';
import HeaderBar from './components/HeaderBar.vue';
import PageAccueil from './pages/PageAccueil.vue'
import PageMateriel from './pages/PageMateriel.vue';

const current_page = ref(null);
</script>

<template>
    <div class="app">
        <HeaderBar />
        
        <div class="layout">
            <NavBar v-model="current_page"/>

            <PageMateriel v-if="current_page !== null" />
            <PageAccueil v-else/>
        </div>
    </div>
</template>

<style scoped>
.app {
    display: flex;
    flex-direction: column;
    height: 100vh;
}

.layout {
    display: flex;
    flex: 1;
    overflow: hidden;
}
</style>
