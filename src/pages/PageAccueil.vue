<script setup>
import { useBreadcrumb } from '../composables/breadcrumb';

const { setBreadcrumb } = useBreadcrumb();
setBreadcrumb([
    { label: 'Accueil', page: null },
])
</script>

<template>
    <div class="content">
        <h1>Bonjour Son Des Cimes !</h1>
    </div>
</template>

<style scoped>
.content {
    flex-grow: 1;
    display: flex;
    justify-content: center;
    align-items: center;
}
</style>