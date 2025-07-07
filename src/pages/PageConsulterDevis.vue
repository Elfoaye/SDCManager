<script setup>
import { invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';
import { save } from '@tauri-apps/plugin-dialog';
import { writeFile } from '@tauri-apps/plugin-fs';
import { revealItemInDir } from '@tauri-apps/plugin-opener';
import { ref, onMounted, computed, watch, h } from 'vue';
import { useToast } from "vue-toastification";
import { useDevisStore } from '../composables/devisStore';
import { useBreadcrumb } from '../composables/breadcrumb';
import DisplayDevis from '../components/DisplayDevis.vue';
import html2pdf from 'html2pdf.js'

const { document, setDocument, setPage } = defineProps(['document', 'setDocument', 'setPage']);

const store = useDevisStore();
const toast = useToast();

const isAdmin = ref(false);
listen('log_in_admin', (event) => {
    isAdmin.value = event.payload;
});

const { setBreadcrumb } = useBreadcrumb();
setBreadcrumb([
    { label: 'Accueil', page: null },
    { label: 'Devis & Factures', page: 'devparcour' },
    { label: 'Consulter', page: 'devconsult' }
]);

const confirm = ref(null);
const badValues = ref([]);
const devisRef = ref(null);

const confirmMessage = computed(() => {
    if(confirm.value === 'duplicate') return 'dupliquer'; 
    if(confirm.value === 'facture') return 'créer une facture depuis';
    return 'suprimer';
});

const titleClass = computed(() => {
    if(store.isFacture)
        return null; 
    if(store.devisInfos.type.includes("valide")) 
        return 'valid';

    return 'in-progress';
});

async function duplicateDevis() {
    try {
        const newId = await invoke("duplicate_devis", { devisId: store.devisInfos.id });
        confirm.value = false;
        setDocument({id: newId, facture: false}, false);
    } catch (err) {
        console.error(err + " on duplicating devis " + store.devisInfos.id);
    }
}

async function createFacture() {
    try {
        const newId = await invoke("facture_from_devis", { devisId: store.devisInfos.id });
        confirm.value = false;
        setDocument({id: newId, facture: true}, false);
    } catch (err) {
        console.error(err + " on facturing devis " + store.devisInfos.id);
    }
}

async function deleteDocument() {
    if(!isAdmin) return;

    try {
        if(store.isFacture) {
            await invoke("delete_facture", { factureId: store.devisInfos.id });
        } else {
            await invoke("delete_devis", { devisId: store.devisInfos.id });
        }
        confirm.value = null;
        setPage('devparcour');
    } catch (err) {
        console.error(err + " on deleting document " + store.devisInfos.id);
    }
}

function isValidDateString(dateStr) {
    const regex = /^\d{4}-\d{2}-\d{2}$/;
    if (!regex.test(dateStr)) return false;

    const date = new Date(dateStr);
    return !isNaN(date.getTime());
}

function verifFacture() {
    badValues.value = [];

    if(!store.devisInfos.id) {
        badValues.value.push('Devis invalide, essayez de rafraichir la page');
    }

    if(!store.devisInfos.name) {
        badValues.value.push('Nom du devis');
    }

    if(!store.devisInfos.date || !isValidDateString(store.devisInfos.date)) {
        badValues.value.push('Date');
    }

    if(!Number.isInteger(store.devisInfos.duration) || store.devisInfos.duration <= 0) {
        badValues.value.push('Durée');
    }

    if(!store.clientInfos.name || store.clientInfos.name.trim() === '') {
        badValues.value.push('Nom du client');
    }

    if(badValues.value.length > 0)  {
        confirm.value = 'badvalues';
        return false;
    }

    return true;
}

function setConfirm(value) {
    if(!(store.devisInfos.id > 0)) return;

    if(value === 'facture' && !verifFacture()) return;

    confirm.value = value;
}

function confirmCancel() {
    confirm.value = null;
}

async function generatePdfBlob() {
    const element = devisRef.value.printRoot;
    if (!element) {
        console.warn("Element introuvable");
        return;
    }

    const opt = {
        margin: [0, 0, 0, 0],
        image: { type: 'jpeg', quality: 0.98 },
        html2canvas: {
            scale: 4,
            windowWidth: 794,
            windowHeight: 1123 
        },
        jsPDF: { unit: 'mm', format: 'a4', orientation: 'portrait' }
    };

    return await html2pdf().set(opt).from(element).toPdf().get('pdf');
}

async function savePdf() {
    try {
        const pdf = await generatePdfBlob();
        const blob = await pdf.output('blob');

        const filePath = await save({
            title: "Enregistrer le PDF",
            defaultPath: `${(store.isFacture ? store.devisInfos.id : 'Devis_') + store.devisInfos.name}.pdf`,
            filters: [{ name: "PDF", extensions: ["pdf"] }]
        });

        if (!filePath) {
            console.log("Sauvegarde annulée");
            return;
        }

        const buffer = new Uint8Array(await blob.arrayBuffer());
        await writeFile(filePath, buffer);

        toast.success(
            h('div', {
                style: { display: 'flex', flexDirection: 'column', cursor: 'pointer' },
                async onClick() {
                    await revealItemInDir(filePath);
                }
            }, [
                h('strong', 'PDF enregistré avec succès'),
                h('small', { style: { marginTop: '4px' } }, 'Cliquez ici pour ouvrir le dossier')
            ]),
            {
                timeout: 5000,
                closeOnClick: true,
                draggable: false,
                closeButtonClassName: 'close-button'
            }
        );

    } catch (err) {
        console.error("Erreur lors de la sauvegarde du PDF : ", err);
        toast.error("Erreur lors de la sauvegarde du PDF", {
            timeout: 5000,
            closeOnClick: true
        });
    }
}

async function printPdf() {
    try {
        const pdf = await generatePdfBlob();
        await pdf.autoPrint();

        const blobUrl = URL.createObjectURL(pdf.output('blob'));
        const win = window.open(blobUrl);
        if (win) win.focus();
    } catch (err) {
        console.error("Erreur lors de l'impression du PDF : ", err);
        toast.error("Erreur lors de la l'impression du PDF", {
            timeout: 5000,
            closeOnClick: true
        });
    }
}

onMounted(async () => {
    isAdmin.value = await invoke('is_admin');
    await store.loadDocument(document);
});

watch(() => document, (newDoc) => {
    if (!newDoc || newDoc.id <= 0) {
        return;
    }

    store.loadDocument(newDoc);
});
</script>

<template>
    <div class="all">
        <div class="content"> 
            <div v-if="confirm" class="confirm">
                <div class="pop-up">
                    <p v-if="confirm === 'delete' && !isAdmin">Les droits admin sont nécessaires pour supprimer un document</p>
                    <p v-else-if="confirm === 'badvalues'" class="bad-val">
                        Les champs suivants ne sont pas valides pour créer une facture :
                        <span v-for="value in badValues"> - {{ value }}</span>
                    </p>
                    <p v-else>Êtes-vous sûr de vouloir 
                        {{ confirmMessage }} 
                        <span>{{ store.devisInfos.name }}</span> ?
                    </p>
                    <p v-if="confirm === 'facture'">(Créer une facture applique la sortie du materiel sélectionné)</p>
                    <p v-if="confirm === 'delete' && isAdmin">(Cette action est irréversible)</p>

                    <div class="confirm-buttons">
                        <button v-if="confirm === 'duplicate'" @click="duplicateDevis" class="new" >Dupliquer</button>
                        <button v-else-if="confirm === 'facture'" @click="createFacture" class="new" >Facturer</button>
                        <button v-else-if="confirm === 'delete' && isAdmin" @click="deleteDocument" class="delete" >Supprimer</button>
                        <button @click="confirmCancel" class="cancel">Annuler</button>
                    </div>
                </div>
            </div>
            <div class="title" :class="titleClass">
                <h1>Consulter {{ store.isFacture ? 'la facture' :'le devis' }}</h1>
            </div>
            <h2>{{ store.devisInfos.type + ' ' + store.devisInfos.id + ' ' + store.devisInfos.name }}</h2>
        </div>
        <section class="preview">
                <DisplayDevis ref="devisRef"/>
            </section>
        <section class="submit">
            <div class="buttons">
                <button v-if="!store.isFacture" class="modif" @click="setDocument({id: store.devisInfos.id, facture: false}, true)">
                    Modifier
                </button>
                <button class="new" @click="savePdf">
                    Télecharger
                </button>
                <button class="new" @click="printPdf">
                    Imprimer
                </button>
            </div>
            <div class="buttons">
                <button v-if="!store.isFacture" @click="setConfirm('duplicate')">
                    Dupliquer
                </button>
                <button v-if="!store.isFacture" @click="setConfirm('facture')">
                    Facturer
                </button>
                <button @click="setConfirm('delete')" class="delete">
                    Supprimer
                </button>
            </div>
        </section>
        </div>
</template>

<style scoped>
.all {
    display: flex;
    flex-direction: column;
    align-items: center;
    width: 100%;
}

.confirm {
    position: absolute;
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
    gap: 1rem;
    height: fit-content;
    background-color: var(--background-alt);
    border: 1px solid var(--border-accent);
    border-radius: 0.5rem;
    padding: 1rem;
    padding-bottom: 2rem;
}

.pop-up p {
    margin: 0 1rem;
}

.confirm-buttons {
    display: flex;
    gap: 1rem;
}

.title.valid {
    border: 2px solid var(--success);
}

.title.in-progress {
    border: 2px solid var(--warning);
}

.bad-val {
    display: flex;
    flex-direction: column;
}

h2 {
    text-transform: capitalize;
    margin-bottom: 1rem;
}

.preview {
    display: flex;
    justify-content: center;
    width: 100%;
}

.submit {
    position: sticky;
    bottom: -0.5rem;
    margin: 0;
    padding: 1rem;
    width: 100%;
    display: flex;
    align-items: center;
    justify-content: center;
    border-bottom: 0;
    gap: 1rem;
    border: 1px solid var(--border-accent);
    background-color: var(--background);
    
}

.buttons {
    display: flex;
    justify-content: center;
    gap: 1rem;
    max-width: 100%;
}

@media screen and (max-width: 1080px) {
    .submit {
        flex-direction: column;
    }
}
</style>