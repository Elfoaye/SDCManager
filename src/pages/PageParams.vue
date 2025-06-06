<script setup>
import { useBreadcrumb } from '../composables/breadcrumb';
import { useTheme } from '../composables/useTheme'
import { computed } from 'vue';

const { setBreadcrumb } = useBreadcrumb();
setBreadcrumb([
    { label: 'Accueil', page: null },
    { label: 'Paramètres', page: 'params' }
]);

const { theme, activeTheme, setTheme } = useTheme();

function toggleTheme() {
    const newTheme = activeTheme.value === 'light' ? 'dark' : 'light';
    setTheme(newTheme);
}

function toggleSystem() {
    const newTheme = theme.value === 'system' ? activeTheme.value : 'system';
    setTheme(newTheme);
}

</script>

<template>
    <div class="content">
        <div class="title">
            <h1>Paramètres</h1>
        </div>
        <div class="params">
            <section>
                <h2>Visuels</h2>
                <label>Thème sombre : <input :checked="activeTheme === 'dark'" @change="toggleTheme" type="checkbox"></label>
                <label>Thème du système : <input :checked="theme === 'system'" @change="toggleSystem" type="checkbox"></label>
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

label:hover {
    cursor: pointer;
}
</style>