<script setup>
import { invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';
import { save } from '@tauri-apps/plugin-dialog';
import { writeFile } from '@tauri-apps/plugin-fs';
import { open } from '@tauri-apps/plugin-shell';
import { dirname } from '@tauri-apps/api/path';
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
const devisRef = ref(null);

const confirmMessage = computed(() => {
    if(confirm.value === 'duplicate') return 'dupliquer'; 
    if(confirm.value === 'facture') return 'créer une facture depuis';
    return 'suprimer';
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

function setConfirm(value) {
    if(!(store.devisInfos.id > 0)) return;

    confirm.value = value;
}

function confirmCancel() {
    confirm.value = null;
}

async function generatePDF() {
    try {
        const element = devisRef.value.printRoot;
        if (!element) {
            console.warn("Element introuvable");
            return;
        }

        const filePath = await save({
            title: "Enregistrer le PDF",
            defaultPath: `${(store.isFacture ? store.devisInfos.id : 'Devis_') + store.devisInfos.name}.pdf`,
            filters: [
                { name: "PDF", extensions: ["pdf"] }
            ]
        });

        if (!filePath) {
            console.log("Sauvegarde annulée");
            return;
        }

        const opt = {
            margin: [0, 0, 0, 0],
            filename: `${(store.isFacture ? store.devisInfos.id : 'Devis_') + store.devisInfos.name}.pdf`,
            image: { type: 'jpeg', quality: 0.98 },
            html2canvas: {
                scale: 4,
                windowWidth: 794,
                windowHeight: 1123 
            },
            jsPDF: { unit: 'mm', format: 'a4', orientation: 'portrait' }
        };

        const pdf = html2pdf().set(opt).from(element).outputPdf('blob');
        const blob = await pdf.outputPdf('blob');

        const buffer = new Uint8Array(await blob.arrayBuffer());

        await writeFile(filePath, buffer);

        
        toast.success(
            h('div',{
                    style: { display: 'flex', flexDirection: 'column', cursor: 'pointer' },
                    async onClick() {
                        const folder = await dirname(filePath);
                        console.log("opening ", folder);
                        await open(folder);
                    }
                },
                [
                    h('strong', 'PDF enregistré avec succès'),
                    h(
                        'small',
                        { style: { marginTop: '4px' } },
                        'Cliquez ici pour ouvrir le dossier'
                    )
                ]
            ),
            {
                timeout: 5000,
                closeOnClick: true,
                draggable: false,
                closeButtonClassName: 'close-button'
            }
        );
    } catch(err) {
        console.error("Erreur lors de la sauvegarde du PDF : " + err);

        toast.error("Erreur lors de la sauvegarde du PDF : " + err, {
            timeout: 5000,
            closeOnClick: true,
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
                    <p>Êtes-vous sûr de vouloir 
                        {{ confirmMessage }} 
                        <span>{{ store.devisInfos.name }}</span> ?
                    </p>

                    <div class="confirm-buttons">
                        <button v-if="confirm === 'duplicate'" @click="duplicateDevis" class="new" >Dupliquer</button>
                        <button v-else-if="confirm === 'delete'" @click="deleteDocument" class="delete" >Supprimer</button>
                        <button v-else @click="createFacture" class="new" >Facturer</button>
                        <button @click="confirmCancel" class="cancel">Annuler</button>
                    </div>
                </div>
            </div>
            <div class="title">
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
                <button class="new" @click="generatePDF">
                    Télecharger
                </button>
                <button v-if="!store.isFacture" @click="setConfirm('duplicate')">
                    Dupliquer
                </button>
                <button v-if="!store.isFacture" @click="setConfirm('facture')">
                    Facturer
                </button>
                <button v-if="isAdmin" @click="setConfirm('delete')" class="delete">
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
    max-height: 5rem;
    background-color: var(--background-alt);
    border: 1px solid var(--border-accent);
    border-radius: 0.5rem;
    padding: 1rem;
    padding-bottom: 2rem;
}

.confirm-buttons {
    display: flex;
    gap: 1rem;
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
    flex-direction: column;
    align-items: center;
    border-bottom: 0;
    border: 1px solid var(--border-accent);
    background-color: var(--background);
    
}

.buttons {
    display: flex;
    justify-content: center;
    gap: 1rem;
    max-width: 100%;
}
</style>