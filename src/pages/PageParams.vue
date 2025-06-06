<script setup>
import { useBreadcrumb } from '../composables/breadcrumb';

const { setBreadcrumb } = useBreadcrumb();
setBreadcrumb([
    { label: 'Accueil', page: null },
    { label: 'Paramètres', page: 'params' }
  ]);

</script>

<template>
    <div class="content">
        <h1>Paramètres</h1>
        <div class="params">
            <section>
                <h2>Visuels</h2>
                <label>Mode sombre : <input type="checkbox"></label>
                <label>Suivre le système : <input type="checkbox"></label>
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