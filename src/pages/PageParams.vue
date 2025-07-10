<script setup>
import { invoke } from '@tauri-apps/api/core';
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

const syncApiKey = ref('');

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

onMounted(() => {
    const confirm = localStorage.getItem('confirmOnClose');
    confirmOnClose.value = confirm !== null ? confirm === 'true' : true;

    const storedFont = localStorage.getItem('fontSize');
    if (storedFont) {
        fontSize.value = storedFont;
        document.documentElement.style.fontSize = `${storedFont}px`;
    }

    if(!syncID) { 
        invoke('get_user_id').then((id) => {syncApiKey.value = id});
    }
});

watch(confirmOnClose, (newVal) => {
    localStorage.setItem('confirmOnClose', newVal.toString());
});

watch(fontSize, (newSize) => {
    localStorage.setItem('fontSize', newSize);
    document.documentElement.style.fontSize = `${newSize}px`;
});
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
                <h2>Synchronisation</h2>
                <p>ID : {{ displayedSyncID }}</p>
            </section>
        </div>
    </div>
</template>

<style scoped>
section {
    display: flex;
    flex-direction: column;
    margin: 1rem;
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

label:hover, input:hover {
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


select {
    padding: 0.5rem;
    color: var(--text);
    background-color: var(--background-alt);
    border: 1px solid var(--border);
    border-radius: 0.3rem;
}
</style>