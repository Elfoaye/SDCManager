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

const newTag = ref('')
const editIndex = ref(null);
const applyMessage = ref({class: 'success', message: ''});

function addType() {
    const tag = newTag.value.trim();
    if (tag && !types.value.includes(tag)) {
        types.value.push(tag);
        console.log("Added " + tag);
    }
    newTag.value = '';
}

function editType(index) {
    editIndex.value = index;
}

function removeType(index) {
    types.value.splice(index, 1);
}

async function applyChanges() {
    try {
        await invoke('set_loc_formulas', { formulas: formulas.value });
        applyMessage.value = {class: 'success', message: "Changements appliqués"};
    } catch (err) {
        applyMessage.value = {class: 'error', message: err};
        console.error(err);
    }
}

function cancelChanges() {
    invoke('get_loc_formulas').then((data) => formulas.value = data);
    invoke('get_materiel_types').then((data) => types.value = data);
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
                <div class="types">
                    <h3>Types (Double click pour modifier)</h3>
                    <ul>
                        <li class="new">
                            <input
                                v-model="newTag"
                                @keyup.enter="addType"
                                placeholder="Ajouter un tag..."
                            />
                            <button @click="addType">+</button>
                        </li>
                        <li v-for="(type, index) in types" :key="index" :class="{ edit: editIndex === index }">
                            <span v-if="editIndex !== index" class="tag" @dblclick="editType(index)">
                                {{ type }}
                            </span>
                            <input v-else
                                v-model="types[index]"
                                @blur="editIndex = null"
                                @keyup.enter="editIndex = null"
                                class="tagEdit" 
                            />
                            <button class="delete" @click="removeType">x</button>
                        </li>
                    </ul>
                </div>
                <div class="submit">
                    <button @click="applyChanges">Appliquer</button>
                    <button @click="cancelChanges">Annuler</button>
                </div>
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
    padding: 1rem;
    border: 1px solid var(--border);
    border-radius: 0.5rem;
    
}

section div:not(.submit) {
    border-bottom: 1px solid var(--border);
    margin-bottom: 1rem;
    padding-bottom: 1rem;
}

label {
    width: 100%;
    text-wrap: nowrap;
}

ul {
  list-style: none;
  padding: 0;
  margin: 0;
  display: flex;
  flex-wrap: wrap;
  gap: 0.5rem;
}

li {
  display: flex;
  align-items: center;
  height: fit-content;
  gap: 0.3rem;
  background-color: var(--accent);
  border: 1px solid var(--border);
  border-radius: 1.5rem;
  padding: 0.3rem 0.6rem;
  transition: background-color 0.2s;
}

li:hover {
  background-color: var(--accent-hover);
}

li.new {
  border-style: dashed;
  background-color: transparent;
}

li .tag,
li input.tagEdit {
  font-size: 0.9rem;
  border: none;
  background: transparent;
  outline: none;
}

li .tag {
  cursor: pointer;
  display: inline-block;
  vertical-align: middle;
}

li.edit {
    background-color: var(--surface-hover);
    border: 1px solid var(--border-accent);
}

input.tagEdit {
  padding: 0.2rem;
  border-radius: 0.4rem;
}

li button {
    background-color: var(--accent);
    width: fit-content;
    height: fit-content;
    color: var(--text);
    border: none;
    padding: 0.3rem;
    cursor: pointer;
    transition: background-color 0.2s;
}

li button:hover {
    background-color: var(--accent-hover);
}

li button.delete {
    background-color: var(--accent);
}

li button.delete:hover {
  background-color: var(--accent-hover);
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