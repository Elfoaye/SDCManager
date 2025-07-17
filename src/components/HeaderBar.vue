<script setup>
import { ref, onMounted } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';
import { useBreadcrumb } from '../composables/breadcrumb';
import { useHasUploaded } from '../composables/hasUploadedStore';
const { breadcrumb } = useBreadcrumb();
const { hasUploaded, syncError, setHasUploaded, setSyncError } = useHasUploaded();

const props = defineProps(['setPage']);
const emit = defineEmits(['cancel']);

const isAdmin = ref(false);
const confirm = ref(false);

const isSyncedDrive = ref(false);
const hasUploadedChanges = ref(hasUploaded);
const isUploading = ref(false);
const isDownloading = ref(false);
const syncResult = ref('');

async function logOutAdmin() {
    await invoke('log_out_admin');
    confirm.value = false;
}

async function sendDataToDrive() {
    if(isDownloading.value || isUploading.value) return;

    isUploading.value = true;
    try {       
        await invoke("upload_sync_data_to_drive");     
        setSyncError(''); 
        setHasUploaded(true);  
    } catch (err) {
        console.error("Erreur lors de l'envoi des données : ", err);
         setSyncError("Erreur lors de l'envoi des données : " + err);
        syncResult.value = err;
    }
    isUploading.value = false;
}

async function getDataFromDrive() {
    if(isDownloading.value || isUploading.value) return;

    isDownloading.value = true;
    try {
        await invoke("download_sync_data_from_drive", { force: false});
        setSyncError('');
    } catch (err) {
        console.error("Erreur lors de la récupération des données : ", err);
        setSyncError("Erreur lors de la récupération des données : " + err);
        syncResult.value = err;
    }
    isDownloading.value = false;
}

listen('log_in_admin', (event) => {
    isAdmin.value = event.payload;
});

listen('set_up_sync', () => {
    invoke("is_synced_to_drive").then((value) => isSyncedDrive.value = value);
});

window.addEventListener('storage', (event) => {
    console.log("Storage update");
    if (event.key === 'uploadedLatest') {
        hasUploadedChanges.value = event.newValue;
        console.log("Changed upload : ", hasUploadedChanges.value);
    }
});

onMounted(() => {
    invoke('is_admin').then((value) => isAdmin.value = value);
    invoke("is_synced_to_drive").then((value) => isSyncedDrive.value = value);
});
</script>

<template>
    <div v-if="confirm" class="confirm">
        <div class="pop-up">
            <p>Êtes-vous sûr de vouloir quitter le <span>Mode Admin</span> ?</p>

            <div class="confirm-buttons">
                <button @click="logOutAdmin" class="submit">Confirmer</button>
                <button @click="confirm=false" class="cancel">Annuler</button>
            </div>
        </div>
    </div>
    <header>
        <div class="topnav" :class="{ admin: isAdmin }">
            <img src="../assets/Logo.png">

            <button v-if="isAdmin" @click="confirm=true">Mode Admin</button>
        </div>
        <nav class="path">
            <button @click="emit('cancel')">&#8617;</button>
            <div
                v-for="(step, index) in breadcrumb"
                :key="index"
                class="step"
            >
                <button @click="setPage(step.page)">{{ step.label }}</button> 
                <span v-if="index < breadcrumb.length - 1">></span>
            </div>
        </nav>
        <div v-if="isSyncedDrive" class="sync">
            <div class="sync-status">
                <span v-if="syncError" class="tooltip error" data-tooltip="Erreur : {{ syncError }}">&#10060;</span>
                <span v-else-if="hasUploadedChanges" class="tooltip success" data-tooltip="Pas de changements non envoyés">&#10003;</span>
                <span v-else class="tooltip warning" data-tooltip="Changements locaux non envoyés">&#9888;</span>
            </div>
            <p>Sync : </p>
            <button @click="getDataFromDrive" :class="{ disabled: isDownloading }">Mettre à jour</button>
            <button @click="sendDataToDrive" :class="{ disabled: isUploading }">Envoyer</button>
        </div>
    </header>
</template>

<style scoped>
header {
    display: flex;
    align-items: center;
    border-bottom: 1px solid var(--border);
    height: clamp(3rem, 8vh, 5rem);
}

.topnav {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    width: clamp(9rem, 18vw, 14rem);
    padding-left: 1rem;
    height: 100%;
}

.topnav img {
    max-height: 100%;
}

.topnav.admin {
    box-sizing: border-box;
    background: linear-gradient(to left, var(--background), var(--warning));
}

.path {
    max-height: 100%;
}

.path button {
    border: 0;
    hyphens: auto;
    white-space: normal;
    font-size: clamp(0.6rem, 2vw, 1rem);
    background-color: var(--background);
    max-height: 90%;
    width: fit-content;

    transition: all 0.15s;
}

.path button:hover {
    background-color: var(--accent);

    transition: all 0.15s;
}

.admin button {
    display: flex;
    align-items: center;
    max-height: 80%;
    width: auto;
    padding: 0.5rem;
    border: 2px solid var(--warning);
    background-color: var(--background);
    border-radius: 2rem;
    font-weight: 600;
}

.admin button:hover {
    background-color: var(--disabled);
    border-color: var(--background-error);
}

.path {
    display: flex;
    max-height: 100%;
}

.confirm {
    position: fixed;
    top: 0;
    left: 0;
    right: 0;
    bottom: 0;
    display: flex;
    justify-content: center;
    align-items: center;
    background-color: rgba(0, 0, 0, 0.5);
    z-index: 1000;
}

.pop-up {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    max-height: 5rem;
    background-color: var(--background-alt);
    border: 1px solid var(--border-accent);
    border-radius: 0.5rem;
    padding: 1em;
    padding-bottom: 2rem;
}

.pop-up button {
    height: 100%;
    padding: 1rem;
    margin-right: 1rem;
    width: 8rem;
    border-radius: 0.5rem;

    transition: all 0.2s;
}

.submit {
    background-color: var(--accent-hover);
}

.cancel {
    background-color: var(--disabled);
}

.sync {
    display: flex;
    align-items: center;
    padding: 0.25rem 0.5rem;
    margin-left: auto;
    margin-right: 0.5rem;
    height: 70%;
    gap: 0.5rem;
    border: 1px solid var(--border);
    border-radius: 0.5rem;
}

.sync button {
    display: flex;
    justify-content: center;
    align-items: center;
    width: clamp(5rem, 10vw, 8rem);
    height: 4rem;
    max-height: 100%;
}

.sync-status {
    font-size: 1.5rem;
    padding: 0 0.5rem;
}

.tooltip {
    position: relative;
    cursor: help;
}

.tooltip::after {
    content: attr(data-tooltip);
    position: absolute;
    top: 125%;
    right: 0;
    transform: translateX(50%);
    background-color: #333;
    color: #fff;
    white-space: nowrap;
    padding: 0.4rem 0.6rem;
    border-radius: 0.3rem;
    font-size: 0.75rem;
    opacity: 0;
    pointer-events: none;
    transition: opacity 0.2s;
    z-index: 10;
}

.tooltip:hover::after {
    opacity: 1;
}

.warning {
    color: var(--warning);
}

.disabled {
    background-color: var(--disabled);
}

.disabled:hover {
    cursor: default;
}
</style>