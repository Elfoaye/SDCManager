<script setup>
import { invoke } from '@tauri-apps/api/core';
import { openUrl } from '@tauri-apps/plugin-opener';
import { useBreadcrumb } from '../composables/breadcrumb';
import { useTheme } from '../composables/useTheme'
import { ref, computed, watch, onMounted } from 'vue';
import { useHasUploaded } from '../composables/hasUploadedStore';

const { setHasUploaded, setSyncError } = useHasUploaded();

const { syncID } = defineProps(['syncID']);

const { setBreadcrumb } = useBreadcrumb();
setBreadcrumb([
    { label: 'Accueil', page: null },
    { label: 'Paramètres', page: 'params' }
]);

const { theme, activeTheme, setTheme } = useTheme();

const activeSyncthing = ref(false);
const syncApiKey = ref('');
const tempPeerSyncName = ref('');
const tempPeerSyncID = ref('');
const addSyncMessage = ref({ class: '', message: '' });
const copyMessage = ref({ class: '', message: '' }); 

const isSyncedDrive = ref(false);
const authURL = ref('');
const addTokenResult = ref({ class: '', message: '' });
const forceDownload = ref(false);
const isUploading = ref(false);
const isDownloading = ref(false);
const syncResult = ref({ class: '', message: '' });

const confirmOnClose = ref(true);
const fontSize = ref('16');

const displayedSyncID = computed(() => {
  return syncID && syncID !== ''
    ? syncID
    : syncApiKey && syncApiKey !== ''
      ? syncApiKey
      : 'Indisponible, essayez de recharger la page';
});

function toggleTheme() {
    const newTheme = activeTheme.value === 'light' ? 'dark' : 'light';
    setTheme(newTheme);
}

function toggleSystem() {
    const newTheme = theme.value === 'system' ? activeTheme.value : 'system';
    setTheme(newTheme);
}

function copyID() {
    if(!displayedSyncID.value) return;

    navigator.clipboard.writeText(displayedSyncID.value)
    .then(() => {copyMessage.value = { class: 'success', message: 'ID copié !'}})
    .catch(() => {copyMessage.value = { class: 'error', message: "Erreur lors de la copie de l'id"}})
}

const openSyncthingUI = async () => {
    await openUrl('http://127.0.0.1:8384/');
};

async function addSyncPeer() {
    if(!tempPeerSyncID.value || !tempPeerSyncName.value) {
        addSyncMessage.value = { class:"error", message: "Veuillez remplir le nom et l'ID pour ajouter un pair" };
        return;
    }

    try {
        await invoke('add_user_to_sync_to', { id: tempPeerSyncID.value, name: tempPeerSyncName.value });
        addSyncMessage.value = { class: 'success', message: `Synchronisation à ${tempPeerSyncName.value} active`};
        tempPeerSyncID.value = '';
        tempPeerSyncName.value = '';
    } catch (err) {
        console.error("Erreur lors de l'ajout d'un peer : ", err);
        addSyncMessage.value = { class:"error", message: err };
    }
}

async function authGoogle() {
    try {
        const url = await invoke("get_google_auth_url");
        openUrl(url);
    } catch (err) {
        console.error("Erreur lors de l'ouverture de l'auth Google : ", err);
        addTokenResult.value = { class:"error", message: err };
    }
}

async function sendDataToDrive() {
    if(isDownloading.value || isUploading.value) return;

    isUploading.value = true;
    try {       
        await invoke("upload_sync_data_to_drive");
        syncResult.value = { class:"success", message: "Données envoyées au drive !" };   
        setSyncError('');
        setHasUploaded(true);  
    } catch (err) {
        console.error("Erreur lors de l'envoi des données : ", err);
        setSyncError("Erreur lors de l'envoi des données : " + err);
        syncResult.value = { class:"error", message: err };
    }
    isUploading.value = false;
}

async function getDataFromDrive(eraseLocal) {
    if(isDownloading.value || isUploading.value) return;

    isDownloading.value = true;
    try {
        await invoke("download_sync_data_from_drive", { force: eraseLocal });
        syncResult.value = { class:"success", message: "Données locales mise à jour !" };
        setSyncError('');
    } catch (err) {
        console.error("Erreur lors de la récupération des données : ", err);
        setSyncError("Erreur lors de la récupération des données : " + err);
        syncResult.value = { class:"error", message: err };
    }
    isDownloading.value = false;
}

async function getTokenFromURL() {
    if(!authURL.value) return;

    try {
        await invoke("save_tokens_from_url", {authUrl : authURL.value});
        addTokenResult.value = { class:"success", message: "Identifiants de synchronisation ajoutés !" };
        syncResult.value = { class: '', message: '' };
        await getDataFromDrive(true);
        invoke("is_synced_to_drive").then((value) => { isSyncedDrive.value = value});

    } catch (err) {
        console.error("Erreur lors de l'obtetion des tokens : ", err);
        addTokenResult.value = { class:"error", message: err };
    }
}

onMounted(() => {
    const confirm = localStorage.getItem('confirmOnClose');
    confirmOnClose.value = confirm !== null ? confirm === 'true' : true;

    const storedFont = localStorage.getItem('fontSize');
    if (storedFont) {
        fontSize.value = storedFont;
        document.documentElement.style.fontSize = `${storedFont}px`;
    }

    invoke("is_synced_to_drive").then((value) => isSyncedDrive.value = value);

    // activeSyncthing.value = localStorage.getItem('activeSyncthing');
    // if(activeSyncthing.value && !syncID) { 
    //     invoke('get_user_id').then((id) => {syncApiKey.value = id});
    // }
});

watch(confirmOnClose, (newVal) => {
    localStorage.setItem('confirmOnClose', newVal.toString());
});

watch(fontSize, (newSize) => {
    localStorage.setItem('fontSize', newSize);
    document.documentElement.style.fontSize = `${newSize}px`;
});

// watch(activeSyncthing, (newValue) => {
//     localStorage.setItem('activeSyncthing', newValue);
// });
</script>

<template>
    <div class="content">
        <div class="title">
            <h1>Paramètres</h1>
        </div>
        <div class="params">
            <section>
                <h2>Visuels</h2>
                <label><input :checked="activeTheme === 'dark'" @change="toggleTheme" type="checkbox"> Thème sombre</label>
                <label><input :checked="theme === 'system'" @change="toggleSystem" type="checkbox"> Thème du système</label>
                <label class="text-size">
                    Taille du texte :
                    <select v-model="fontSize">
                    <option value="14">Petit</option>
                    <option value="16">Normal</option>
                    <option value="18">Grand</option>
                    <option value="20">Très grand</option>
                    </select>
                </label>
                <p class="note" v-if="fontSize > 16">(Attention: cette taille de texte n'est pas prévue pour les petites fenêtres)</p>
            </section>
            <section>
                <h2>Interactions</h2>
                <label><input type="checkbox" v-model="confirmOnClose" /> Confirmer avant de quitter l’application</label>
            </section>
            <section>
                <h2>Synchronisation Drive</h2>
                <p v-if="isSyncedDrive" class="sync-info active">&#10003; Synchronisation mise en place</p>
                <p v-else class="sync-info">&#10060; Synchronisation désactivée</p>
                <div v-if="isSyncedDrive">
                    <h3>Synchronisation manuelle</h3>
                    <div class="sync-buttons">
                        <button @click="getDataFromDrive(forceDownload)" :class="{ disabled: isDownloading }">
                            Mettre à jour les données locales
                        </button>
                        <button @click="sendDataToDrive" :class="{ disabled: isUploading }">
                            Envoyer les données au drive
                        </button>
                    </div>
                    <label class="option"><input type="checkbox" v-model="forceDownload" />Forcer la mise à jour (&#9888; écrase les données locales)</label>
                    <p v-if="syncResult.message" :class="syncResult.class">{{ syncResult.message }}</p>
                </div>
                <div>
                    <h3>Mise en place de la synchronisation</h3>
                    <div class="guide">
                        <h3>Guide: Mettre en place la synchronisation</h3>
                        <p>
                            1 - Cliquez sur le bouton "Connexion Google" ci-desous. Cela devrait ouvrir votre navigateur.<br>
                            2 - Connectez vous à votre compte Google, puis cliquez sur "Continuer" jusqu'à que vous arriviez sur une page qui ne charge pas (Qui affiche quelque chose comme "Impossible de se connecter").<br>
                            3 - Copiez l'URL de la page EN ENTIER (Il devrait commencer par http://localhost/?code=).<br>
                            4 - Collez l'URL dans le champ "URL D'authentifiaction" ci-dessous, et cliquez sur "Ajouter".<br>
                            <span>&#9888; Cette étape télechargera les données du drive et écrasera les données locales</span><br>
                            5 - La synchronisation devrait être en place !<br>
                            L'app est synchronisée automatiquement à chaque démarrage. Pensez à télécharger les données du drive avant chaque changement, 
                            et envoyer vos données quand vos changements sont terminés !
                        </p>
                    </div>
                    <button @click="authGoogle">Connexion Google</button>
                    <div class="id">
                        <label>URL D'authentifiaction : </label><input v-model="authURL"/>
                        <button @click="getTokenFromURL">
                            Ajouter
                        </button>
                    </div>
                    <p v-if="addTokenResult.message" :class="addTokenResult.class">{{ addTokenResult.message }}</p>
                </div>
            </section>
            <!-- <section>
                <h2>Synchronisation Peer To Peer</h2>
                <label><input v-model="activeSyncthing" type="checkbox"/> Activer la Synchronisation P2P (Requiert un redémarage pour appliquer)</label>
                <div v-if="activeSyncthing == true" class="synthing-options">
                    <div class="id">
                        <label>Sync ID : </label><input :value="displayedSyncID" readonly />
                        <button @click="copyID">
                            Copier
                        </button>
                        <p v-if="copyMessage.message" :class="copyMessage.class">{{ copyMessage.message }}</p>
                    </div>
                    <div class="add-sync">
                        <p>Ajouter un utilisateur à la connexion : </p>
                        <p class="error">Attention : Ajouter un utilisateur écrasera vos données, assurez vous que ses données soient à jour avant de l'ajouter.</p>
                        <label>Nom : <input class="text" v-model="tempPeerSyncName"/></label>
                        <label>ID : <input class="text" v-model="tempPeerSyncID"/></label>
                        <button @click="addSyncPeer">Ajouter</button>
                        <p v-if="addSyncMessage.message" :class="addSyncMessage.class">{{ addSyncMessage.message }}</p>
                    </div>
                    <button class="syncthing" @click="openSyncthingUI">Interface Syncthing</button>
                </div>
            </section> -->
        </div>
    </div>
</template>

<style scoped>
section {
    display: flex;
    flex-direction: column;
    padding: 1rem;
    margin-bottom: 1rem;
    border: 1px solid var(--border);
    border-radius: 0.5rem;
}

h2 {
    margin-bottom: 0.5rem;
}

label {
    width: fit-content;
    font-size: 1rem;
}

input {
    accent-color: var(--accent);
}

input:hover {
    accent-color: var(--accent-hover);
}

label:hover, input:not(.text):hover {
    cursor: pointer;
}

.text-size {
    margin-top: 1rem;
}

.note {
    opacity: 80%;
    font-size: 0.85rem;
    margin: 0;
}

.sync-info {
    width: fit-content;
    font-weight: 500;
    padding: 1rem;
    margin: 0;
    border: 1px solid var(--error);
    border-radius: 0.5rem;
}

.sync-info.active {
    border: 1px solid var(--success);
}

.synthing-options {
    display: flex;
    flex-direction: column;
}

.add-sync {
    display: flex;
    flex-direction: column;
    gap: 1rem;
}

.add-sync p {
    margin: 0;
}

.id {
    width: 100%;
    margin-top: 1rem;
    display: flex;
    align-items: center;
}

.id label {
    margin-right: 1rem;
}

.id input {
    width: 50%;
    max-width: 10rem;
    padding: 0.5rem;
    border-top-right-radius: 0;
    border-bottom-right-radius: 0;
    overflow-x: auto;
    white-space: nowrap;
    text-overflow: ellipsis;
    margin-right: 0;
}

.id input:hover {
    cursor: text;
}

.id button {
    padding: 0.4rem;
    flex: 1;
    max-width: 4rem;
    margin-left: 0;
    border-left: 0;
    border-top-right-radius: 0.3rem;
    border-bottom-right-radius: 0.3rem;
    border-top-left-radius: 0;
    border-bottom-left-radius: 0;
}

.id p {
    padding-left: 1rem;
}

.disabled {
    background-color: var(--disabled);
}

.disabled:hover {
    cursor: default;
}

.guide {
    padding-left: 1rem;
    margin-bottom: 1rem;
    border: 1px solid var(--border);
    border-radius: 0.5rem;
}

.sync-buttons {
    display: flex;
    gap: 1rem;
    height: 4rem;
    margin-bottom: 1rem;
}

.sync-buttons button {
    flex: 1;
    max-width: 12rem;
    display: flex;
    justify-content: center;
    align-items: center;
}

h3 {
    margin-top: 1rem;
    margin-bottom: 1rem;
}

select {
    padding: 0.5rem;
    color: var(--text);
    background-color: var(--background-alt);
    border: 1px solid var(--border);
    border-radius: 0.3rem;
}

.syncthing {
    margin-top: 1rem;
}
</style>