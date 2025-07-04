<script setup>
import { useBreadcrumb } from '../composables/breadcrumb';
import { useTheme } from '../composables/useTheme'
import { ref, watch, onMounted } from 'vue';

const { setBreadcrumb } = useBreadcrumb();
setBreadcrumb([
    { label: 'Accueil', page: null },
    { label: 'Paramètres', page: 'params' }
]);

const { theme, activeTheme, setTheme } = useTheme();

const confirmOnClose = ref(true);
const fontSize = ref('16');

function toggleTheme() {
    const newTheme = activeTheme.value === 'light' ? 'dark' : 'light';
    setTheme(newTheme);
}

function toggleSystem() {
    const newTheme = theme.value === 'system' ? activeTheme.value : 'system';
    setTheme(newTheme);
}

onMounted(() => {
  const stored = localStorage.getItem('confirmOnClose');
  confirmOnClose.value = stored !== null ? stored === 'true' : true;
});

onMounted(() => {
  const stored = localStorage.getItem('fontSize');
  if (stored) {
    fontSize.value = stored;
    document.documentElement.style.fontSize = `${stored}px`;
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
                <label>
                    Taille du texte :
                    <select v-model="fontSize">
                    <option value="14">Petit</option>
                    <option value="16">Normal</option>
                    <option value="18">Grand</option>
                    <option value="20">Très grand</option>
                    </select>
                </label>
            </section>
            <section>
                <h2>Interactions</h2>
                <label><input type="checkbox" v-model="confirmOnClose" /> Confirmer avant de quitter l’application</label>
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

label:hover, input:hover {
    cursor: pointer;
}
</style>