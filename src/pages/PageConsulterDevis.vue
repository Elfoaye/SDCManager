<script setup>
import { invoke } from '@tauri-apps/api/core';
import { save } from '@tauri-apps/plugin-dialog';
import { writeFile } from '@tauri-apps/plugin-fs';
import { ref, onMounted } from 'vue';
import { useDevisStore } from '../composables/devisStore';
import { useBreadcrumb } from '../composables/breadcrumb';
import html2pdf from 'html2pdf.js'
import DisplayDevis from '../components/DisplayDevis.vue';

const { devis, setDevis } = defineProps(['devis', 'setDevis']);

const store = useDevisStore();

const { setBreadcrumb } = useBreadcrumb();
setBreadcrumb([
    { label: 'Accueil', page: null },
    { label: 'Devis & Factures', page: 'devparcour' },
    { label: 'Consulter', page: 'devconsult' }
]);

const confirm = ref(null);
const devisRef = ref(null);

async function duplicateDevis() {
    try {
        const newId = await invoke("duplicate_devis", { devisId: store.devisInfos.id });
        confirm.value = false;
        setDevis(newId, false);
    } catch (err) {
        console.error(err + " on duplicating devis " + store.devisInfos.id);
    }
}

function confirmDuplicate() {
    confirm.value = 'delete';
}

function confirmCancel() {
    confirm.value = null;
}

function generatePDF() {
    const element = devisRef.value.printRoot;
    if (!element) {
        console.warn("Element introuvable");
        return;
    }

    const opt = {
        margin: [0, 0, 0, 0],
        filename: `${store.devisInfos.id + store.devisInfos.name}.pdf`,
        image: { type: 'jpeg', quality: 0.98 },
        html2canvas: {
            scale: 2,
            windowWidth: 794,
            windowHeight: 1123 
        },
        jsPDF: { unit: 'mm', format: 'a4', orientation: 'portrait' }
    };

    console.log("Saving pdf");
    html2pdf().set(opt).from(element).save();

    // try {
    //     const pdfBuffer = await html2pdf().set(opt).from(element).outputPdf('arraybuffer');
    //     console.log('Type de pdfBuffer:', typeof pdfBuffer);
    //     console.log('Instance de ArrayBuffer:', pdfBuffer instanceof ArrayBuffer);
    //     console.log('Taille du buffer:', pdfBuffer?.byteLength);
        
    //     const path = await save({
    //         filters: [{ name: 'PDF Document', extensions: ['pdf'] }],
    //         defaultPath: `${store.devisInfos.id + store.devisInfos.name}.pdf`
    //     });

    //     if(!path) {
    //         console.log('Sauvegarde annulée');
    //         return;
    //     }

    //     await writeFile({ path: path, contents: new Uint8Array([72, 101, 108, 108, 111]) });
    //     // await writeFile({ path, contents: new Uint8Array(pdfBuffer) });

    //     console.log('PDF sauvegardé dans ', path);
    // } catch (err) {
    //     console.error('Erreur lors de la génération ou sauvegarde du PDF :', err);
    // }
}

onMounted(() => {
    store.loadDevis(devis);
});
</script>

<template>
    <div class="content"> 
        <div v-if="confirm" class="confirm">
            <div class="pop-up">
                <p>Êtes-vous sûr de vouloir dupliquer <span>{{ store.devisInfos.name }}</span> ?</p>

                <div class="confirm-buttons">
                    <button @click="duplicateDevis" class="new" >Dupliquer</button>
                    <button @click="confirmCancel" class="cancel">Annuler</button>
                </div>
            </div>
        </div>
        <div class="title">
            <h1>Consulter le devis</h1>
        </div>
        <h2>{{ store.devisInfos.id + ' ' + store.devisInfos.name }}</h2>
        <section class="preview">
            <DisplayDevis ref="devisRef"/>
        </section>

        <section class="submit">
            <div class="buttons">
                <button class="modif" @click="setDevis(store.devisInfos.id, true)">
                    Modifier
                </button>
                <button class="new" @click="confirmDuplicate">
                    Dupliquer
                </button>
                <button @click="generatePDF">
                    Télecharger
                </button>
                <button>
                    Facturer
                </button>
            </div>
        </section>
    </div>
</template>

<style scoped>
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
    margin-bottom: 1rem;
}

.preview {
    display: flex;
    justify-content: center;
}

.submit {
    padding: 1rem;
}

.submit {
    position: sticky;
    bottom: -0.5rem;
    margin: 0;
    display: flex;
    flex-direction: column;
    align-items: center;
    border-bottom: 0;
    border: 1px solid var(--border-accent);
    border-bottom: 0;
    border-top-left-radius: 0.5rem;
    border-top-right-radius: 0.5rem;
    background-color: var(--background);
}

.buttons {
    display: flex;
    gap: 1rem;
}
</style>