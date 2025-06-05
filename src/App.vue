<script setup>
import { ref } from 'vue';
import { invoke } from '@tauri-apps/api/core';

import NavBar from './components/NavBar.vue';
import HeaderBar from './components/HeaderBar.vue';
import PageAccueil from './pages/PageAccueil.vue'
import PageMateriel from './pages/PageMateriel.vue';
import PageAuth from './pages/PageAuth.vue';

const currentPage = ref(null);
const lastPage = ref(null);
const redirect = ref(null);

async function setPage(value) {
    if (currentPage.value == value) return;

    if(value === 'modif') {
        const admin = await invoke('is_admin');
        if(!admin) {
            lastPage.value = currentPage.value;
            redirect.value = value;
            currentPage.value = 'auth';
            return;
        }
    }

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
                <PageAuth 
                    v-if="currentPage === 'auth'" 
                    :redirect="redirect"
                    :setPage="setPage"
                    @cancel="setPage(lastPage)"
                />

                <PageMateriel 
                    v-else-if="currentPage === 'consult' || currentPage === 'modif'" 
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
    display: flex;
    padding: 1rem;
    overflow: hidden;
}
</style>
