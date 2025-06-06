<script setup>
import { useBreadcrumb } from '../composables/breadcrumb';
import { invoke } from '@tauri-apps/api/core';
import { ref, onMounted } from 'vue';

const { setBreadcrumb } = useBreadcrumb();
setBreadcrumb([
    { label: 'Accueil', page: null },
    { label: 'Admin', page: 'admin' }
  ]);

const formulas = ref({
  contrib_first_day: 0,
  contrib_following: 0
});
invoke('get_loc_formulas').then((data) => formulas.value = data);
const types = ref([]);
invoke('get_materiel_types').then((data) => types.value = data);

const applyMessage = ref({class: 'success', message: ''});

async function applyChanges() {
    try {
        await invoke('set_loc_formulas', { formulas: formulas.value });
        applyMessage.value = {class: 'success', message: "Changements appliqués"};
    } catch (err) {
        applyMessage.value = {class: 'error', message: err};
        console.error(err);
    }
}
</script>

<template>
    <div class="content">
        <div class="title">
            <h1>Page Admin</h1>
        </div>
        <div class="params">
            <section>
                <h2>Materiel</h2>
                <div class="formulas">
                    <h3>Formules</h3>
                    <label>Contribution de base :
                        <input v-model="formulas.contrib_first_day" type="number" />
                    </label>
                    <p>({{ Number((formulas.contrib_first_day*100).toFixed(2)) }}% de la valeur de l'objet)</p>
                    <label>Contribution suivante :
                        <input v-model="formulas.contrib_following" type="number" />
                    </label>
                    <p>({{ Number((formulas.contrib_following*100).toFixed(2)) }}% de la contribution de base par jour supplémentaire)</p>
                </div>
                <div class="Types">
                    <h3>Types</h3>
                    <ul>
                        <li class="new">Ajouter</li>
                        <li v-for="type in types">
                            {{ type }}
                        </li>
                    </ul>
                </div>
                <button @click="applyChanges">Appliquer les changements</button>
                <p v-if="applyMessage.message" :class="applyMessage.class">{{ applyMessage.message }}</p>
            </section>
            <!-- <section>
                <h2>Admin</h2>
                <div class="password">
                    <button>Changer le mot de passe</button>
                </div>
            </section> -->
        </div>
    </div>
</template>

<style scoped>
.content {
    display: flex;
    flex-direction: column;
    margin: 0;
}

.params {
    max-width: 100%;
    display: grid;
    overflow-y: auto;
}

h2 {
    margin-bottom: 1rem;
}

h3 {
    margin-bottom: 0.5rem;
}

section {
    max-width: 100%;
    display: flex;
    flex-direction: column;
    border: 1px solid var(--border);
    border-radius: 0.5rem;
    padding: 1rem;
}

section div {
    border-bottom: 1px solid var(--border);
    margin-bottom: 1rem;
}

label {
    width: 100%;
    text-wrap: nowrap;
}

ul {
    display: flex;
    flex-direction: column;
    list-style-type: none;
    width: fit-content;
    margin: 0;
    padding: 0;
    padding-right: 2rem;
    padding-bottom: 2rem;
    overflow-y: auto;
    overflow-x: hidden;
}

li {
    padding: 0 0.5rem;
    margin: 0;
    border-bottom: 1px solid var(--border);
}

li.new {
    background-color: var(--success);
}

.formulas p {
    margin-top: 0;
}

p.success {
    color: var(--success);
}

p.error {
    color: var(--error);
}
</style>