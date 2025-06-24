<script setup>
import { ref } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';
import { useTheme } from './composables/useTheme'

import NavBar from './components/NavBar.vue';
import HeaderBar from './components/HeaderBar.vue';
import PageAccueil from './pages/PageAccueil.vue'
import PageMateriel from './pages/PageMateriel.vue';
import PageEditerDevis from './pages/PageEditerDevis.vue';
import PageConsulterDevis from './pages/PageConsulterDevis.vue';
import PageParcourirDevis from './pages/PageParcourirDevis.vue';
import PageAuth from './pages/PageAuth.vue';
import PageAdmin from './pages/PageAdmin.vue';
import PageParams from './pages/PageParams.vue';

const { loadTheme } = useTheme()
loadTheme()

const currentPage = ref(null);
const lastPage = ref(null);
const redirect = ref(null);
const currentDevis = ref(0);

const adminPages = ['modif','admin'];
function isAdminProtected(value) {
    return adminPages.includes(value);
}

async function setPage(value) {
    if (currentPage.value == value) return;

    if(isAdminProtected(value)) {
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

function setDevis(id, modif) {
    currentDevis.value = id;

    if(modif || id === 0) {
        setPage('devmodif');
    } else {
        setPage('devconsult');
    } 
}

function onAdminLogOut() {
    if(isAdminProtected(currentPage.value)) {
        setPage(null);
    }
}

listen('log_in_admin', (event) => {
    if(!event.payload) {
        onAdminLogOut();
    }
});
</script>

<template>
    <div class="app">
        <HeaderBar :setPage="setPage"/>
        <div class="layout">
            <NavBar class="no-print" :setPage="setPage"/>

            <div class="page">
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
                <PageParcourirDevis 
                    v-else-if="currentPage === 'devparcour'" 
                    :setDevis="setDevis"
                />
                <PageConsulterDevis 
                    v-else-if="currentPage === 'devconsult'" 
                    :devis="currentDevis"
                    :setDevis="setDevis"
                />
                <PageEditerDevis 
                    v-else-if="currentPage === 'devmodif'" 
                    :devis="currentDevis"
                    :setDevis="setDevis"
                />
                <PageParams v-else-if="currentPage === 'params'" />
                <PageAdmin v-else-if="currentPage === 'admin'" />
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
    width: 100%;
    overflow: hidden;
}

.page {
    flex: 1;
    display: flex;
    justify-content: center;
    padding: 0.5rem;
    overflow-y: auto;
    overflow-x: hidden;
    max-width: 100%;
}
</style>
