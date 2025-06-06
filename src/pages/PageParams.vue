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
        <h1>Paramètres</h1>
        <div class="params">
            <section>
                <h2>Visuels</h2>
                <label>Mode sombre : <input :checked="activeTheme === 'dark'" @change="toggleTheme" type="checkbox"></label>
                <label>Suivre le système : <input :checked="theme === 'system'" @change="toggleSystem" type="checkbox"></label>
            </section>
        </div>
    </div>
</template>

<style scoped>
.content {
    display: flex;
    flex-direction: column;
}
</style>