<script setup>
import { ref, onMounted } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';
import { useTheme } from './composables/useTheme'
import { Window } from '@tauri-apps/api/window';
import { confirm } from '@tauri-apps/plugin-dialog';
import { useHasUploaded } from './composables/hasUploadedStore';

import NavBar from './components/NavBar.vue';
import HeaderBar from './components/HeaderBar.vue';
import PageAccueil from './pages/PageAccueil.vue'
import PageCalendrier from './pages/PageCalendrier.vue';
import PageMateriel from './pages/PageMateriel.vue';
import PageEditerDevis from './pages/PageEditerDevis.vue';
import PageConsulterDevis from './pages/PageConsulterDevis.vue';
import PageParcourirDevis from './pages/PageParcourirDevis.vue';
import PageAuth from './pages/PageAuth.vue';
import PageAdmin from './pages/PageAdmin.vue';
import PageParams from './pages/PageParams.vue';

const { loadTheme } = useTheme()
loadTheme()

const { hasUploaded, setHasUploaded } = useHasUploaded();

const currentPage = ref(null);
const lastPage = ref(null);
const redirect = ref(null);
const currentDocument = ref({id: 0, facture: false});

const syncthingUserID = ref('');

let closeFlag = false;

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

function setDocument(document, modif) {
    currentDocument.value = document;

    if(document.id === 0 || modif) {
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

onMounted(async () => {
    const appWindow = new Window('main');

    appWindow.onCloseRequested(async (event) => {
        const confirmOnClose = localStorage.getItem('confirmOnClose') !== 'false';
        const synced = await invoke("is_synced_to_drive");
        const needConfirm = synced && !hasUploaded.value
        console.log("Synced : " + synced + " Has uploaded : " + hasUploaded.value + " needConfirm : " + needConfirm);
        
        if((!confirmOnClose && !needConfirm) || closeFlag) {
            // await invoke('stop_syncthing');
            return;
        }
        event.preventDefault();


        const shouldClose = needConfirm ? 
            await confirm("Des changements n'ont pas été envoyés au cloud, voulez-vous vraiment quitter l’application ? Ces changements seront perdus.",
                { title: 'Confirmer la fermeture', type: 'warning' }) :
            await confirm('Voulez-vous vraiment quitter l’application ?',
                { title: 'Confirmer la fermeture', type: 'warning' }) ;
        

        if(shouldClose) {
            closeFlag = true;
            // await invoke('stop_syncthing');
            await appWindow.close();
        }
    });
});

onMounted(async () => {
    await invoke("download_sync_data_from_drive", { force: false });
    setHasUploaded(true);
});

// onMounted(() => {
//     const activeSync = localStorage.getItem('activeSyncthing');
//     if(activeSync == true) {
//         invoke("setup_syncthing_sync")
//         .then((id) => { 
//             syncthingUserID.value = id;
//         })
//         .catch((e) => console.error("Erreur Syncthing:", e));
//         console.log("Syncthing setup, activrSync : ", activeSync);
//     }
// });
</script>

<template>
    <div class="app">
        <HeaderBar :setPage="setPage" @cancel="setPage(lastPage)"/>
        <div class="layout">
            <NavBar :setPage="setPage" @cancel="setPage(lastPage)"/>

            <div class="page">
                <PageAuth 
                    v-if="currentPage === 'auth'" 
                    :redirect="redirect"
                    :setPage="setPage"
                />
                <PageMateriel 
                    v-else-if="currentPage === 'consult' || currentPage === 'modif'" 
                    :modif="currentPage === 'modif'"
                    :setDocument="setDocument"
                />
                <PageCalendrier 
                    v-else-if="currentPage === 'cal'"
                    :setDocument="setDocument"
                />
                <PageParcourirDevis 
                    v-else-if="currentPage === 'devparcour'" 
                    :setDocument="setDocument"
                />
                <PageConsulterDevis 
                    v-else-if="currentPage === 'devconsult'" 
                    :document="currentDocument"
                    :setDocument="setDocument"
                    :setPage="setPage"
                />
                <PageEditerDevis 
                    v-else-if="currentPage === 'devmodif'" 
                    :document="currentDocument"
                    :setDocument="setDocument"
                />
                <PageParams 
                    v-else-if="currentPage === 'params'" 
                    :syncID="syncthingUserID"
                />
                <PageAdmin v-else-if="currentPage === 'admin'" />
                <PageAccueil 
                    v-else
                    :setDocument="setDocument"
                    :setPage="setPage"
                />
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
    overflow-y: auto;
    overflow-x: hidden;
    max-width: 100%;
}
</style>
