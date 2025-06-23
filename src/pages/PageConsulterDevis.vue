<script setup>
import { invoke } from '@tauri-apps/api/core';
import { ref, onMounted } from 'vue';
import { useDevisStore } from '../composables/devisStore';
import { useBreadcrumb } from '../composables/breadcrumb';
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

onMounted(() => {
    store.loadDevis(devis);
});
</script>

<template>
    <div class="content"> 
        <div v-if="confirm" class="no-print confirm">
            <div class="pop-up">
                <p>Êtes-vous sûr de vouloir dupliquer <span>{{ store.devisInfos.name }}</span> ?</p>

                <div class="confirm-buttons">
                    <button @click="duplicateDevis" class="new" >Dupliquer</button>
                    <button @click="confirmCancel" class="cancel">Annuler</button>
                </div>
            </div>
        </div>
        <div class="no-print title">
            <h1>Consulter le devis</h1>
        </div>
        <h2 class="no-print">{{ store.devisInfos.id + ' ' + store.devisInfos.name }}</h2>
        <section class="preview">
            <DisplayDevis class="preview-small"/>
        </section>

        <section class="no-print submit">
            <div class="buttons">
                <button class="modif" @click="setDevis(store.devisInfos.id, true)">
                    Modifier
                </button>
                <button class="new" @click="confirmDuplicate">
                    Dupliquer
                </button>
                <button>
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

.preview-small {
    height: fit-content;
    overflow: hidden;
    border: 1px solid var(--border);
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