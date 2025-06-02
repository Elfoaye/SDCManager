<script setup>
import { ref } from 'vue';

import NavBar from './components/NavBar.vue';
import HeaderBar from './components/HeaderBar.vue';
import PageAccueil from './pages/PageAccueil.vue'
import PageMateriel from './pages/PageMateriel.vue';

const currentPage = ref(null);
const lastPage = ref(null);

function setPage(value) {
    if (currentPage.value == value) return;

    lastPage.value = currentPage.value;
    currentPage.value = value;
}
</script>

<template>
    <div class="app">
        <HeaderBar 
            :currentPage="currentPage"
            :lastPage="lastPage"
            :setPage="setPage"
        />
        
        <div class="layout">
            <NavBar :setPage="setPage"/>

            <div class="content">
                <PageMateriel 
                    v-if="currentPage === 'consult' || currentPage === 'modif'" 
                    :modif="currentPage === 'modif'"
                />
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
    flex: 1;
    display: flex;
    overflow: hidden;
}

.content {
    flex: 1;
    padding: 1rem;
    overflow: hidden;
}
</style>
