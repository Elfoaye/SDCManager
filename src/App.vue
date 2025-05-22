<script setup>
import { ref } from 'vue';

import NavBar from './components/NavBar.vue';
import HeaderBar from './components/HeaderBar.vue';
import PageAccueil from './pages/PageAccueil.vue'
import PageMateriel from './pages/PageMateriel.vue';

const current_page = ref('mat');
const last_page = ref(null);

function setPage(value) {
    if (current_page.value == value) return;

    last_page.value = current_page.value;
    current_page.value = value;
    console.log("last page : " + last_page.value + ", current page : " + current_page.value);
}
</script>

<template>
    <div class="app">
        <HeaderBar 
            :current_page="current_page"
            :last_page="last_page"
            :setPage="setPage"
        />
        
        <div class="layout">
            <NavBar :setPage="setPage"/>

            <div class="content">
                <PageMateriel v-if="current_page === 'mat'" />
                <PageAccueil v-else/>
            </div>
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

.content {
    flex: 1;
    padding: 1rem;
    overflow: hidden;
}
</style>
