<script setup>
import { invoke } from '@tauri-apps/api/core';
import { openUrl } from '@tauri-apps/plugin-opener';
import { useBreadcrumb } from '../composables/breadcrumb';
import { useTheme } from '../composables/useTheme'
import { ref, computed, watch, onMounted } from 'vue';

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

const authURL = ref('');
const confirmOnClose = ref(true);
const addTokenResult = ref({ class: '', message: '' });
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

async function getTokenFromURL() {
    if(!authURL.value) return;

    try {
        const tokens = await invoke("save_tokens_from_url", {authUrl : authURL.value});
        console.log(tokens);
        addTokenResult.value = { class:"success", message: "Identifiants de synchronisation ajoutés !" };
    } catch (err) {
        console.error("Erreur lors de l'obtetion des tokens : ", err);
        addTokenResult.value = { class:"error", message: err };
    }
}

async function sendDataToDrive() {
    try {
        await invoke("upload_sync_data_to_drive");
        console.log("Données envoyées au drive");
    } catch (err) {
        console.error("Erreur lors de l'envoi des données : ", err);
    }
}

async function getDataFromDrive() {
    try {
        await invoke("download_sync_data_from_drive", { force: true });
        console.log("Données mise à jour");
    } catch (err) {
        console.error("Erreur lors de la récupération des données : ", err);
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
                <button @click="authGoogle">Connexion Google</button>
                <p>Connectez vous, puis lorsque vous arrivez sur une page qui ne charge pas, copiez l'URL de la page ci-dessous</p>
                <div class="id">
                    <label>URL D'authentifiaction : </label><input v-model="authURL"/>
                    <button @click="getTokenFromURL">
                        Ajouter
                    </button>
                </div>
                <p v-if="addTokenResult.message" :class="addTokenResult.class">{{ addTokenResult.message }}</p>
                <button @click="sendDataToDrive">
                    Envoyer les données
                </button>
                <button @click="getDataFromDrive">
                    Mettre à jour
                </button>
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
    margin: 1rem;
    margin-bottom: 2rem;
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
    margin-bottom: 0.5rem;
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