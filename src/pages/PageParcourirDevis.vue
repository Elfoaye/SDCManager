<script setup>
import { useBreadcrumb } from '../composables/breadcrumb';

const { setBreadcrumb } = useBreadcrumb();
setBreadcrumb([
    { label: 'Accueil', page: null },
    { label: 'Devis & Factures', page: 'devparcour' },
  ]);
</script>

<template>
    <div class="content">
        <div class="title">
            <h1>Parcourir les devis et factures</h1>
        </div>
    </div>
</template>

<style scoped>

</style>