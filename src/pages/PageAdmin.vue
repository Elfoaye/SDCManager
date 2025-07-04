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
    contrib_following: 0,
    transport_km: 0,
    tech_day: 0,
    tech_hour: 0,
    membership: 0
});
invoke('get_loc_formulas').then((data) => formulas.value = data);
const types = ref([]);
invoke('get_materiel_types').then((data) => types.value = data);

const newTag = ref('');
const editIndex = ref(null);
const tagMessage = ref('');

const oldPassword = ref('');
const newPassword = ref('');
const confirmNewPassword = ref('');

const applyMessage = ref({class: 'success', message: ''});

function addType() {
    const tag = newTag.value.trim();
    if (tag && !types.value.includes(tag)) {
        types.value.push(tag);
    }
    newTag.value = '';
    tagMessage.value = 'Pensez à appliquer les changements';
}

function editType(index) {
    editIndex.value = index;
    tagMessage.value = 'Pensez à appliquer les changements';
}

function removeType(index) {
    types.value.splice(index, 1);
    tagMessage.value = 'Pensez à appliquer les changements';
}

async function applyFormulas() {
    if(!formulas.value || formulas.value.contrib_first_day == '' || formulas.value.contrib_following == '')
        return;

    return await invoke('set_loc_formulas', { formulas: formulas.value });
}

async function applyPassword() {
    if(oldPassword.value == '' || newPassword.value == '' || confirmNewPassword.value == '') return;
    if(newPassword.value !== confirmNewPassword.value) return Promise.reject('Les nouveaux mots de passe ne correspondent pas');

    return await invoke('update_admin_password', { oldPassword: oldPassword.value, newPassword: newPassword.value });
}

function resetFields() {
    invoke('get_loc_formulas').then((data) => formulas.value = data);
    invoke('get_materiel_types').then((data) => types.value = data);
    oldPassword.value = '';
    newPassword.value = '';
    confirmNewPassword.value = '';
    applyMessage.value = {class: 'success', message: ''};
    tagMessage.value = '';
}

async function applyChanges() {
    try {
        await invoke('set_materiel_types', { newTypes: types.value });
        await applyFormulas();
        await applyPassword();
        resetFields();
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
                <h3>Formules</h3>
                <div class="formulas">
                    <div class="pack">
                        <label>Contribution de base :
                            <input v-model="formulas.contrib_first_day" type="number" step="0.01"/>
                        </label>
                        <p>({{ Number((formulas.contrib_first_day*100).toFixed(2)) }}% de la valeur de l'objet)</p>
                        <label>Contribution suivante :
                            <input v-model="formulas.contrib_following" type="number" step="0.1"/>
                        </label>
                        <p>({{ Number((formulas.contrib_following*100).toFixed(2)) }}% de la contribution de base par jour supplémentaire)</p>
                    </div>
                    <div class="pack">
                        <label>Transport au kilomètre :
                            <input v-model="formulas.transport_km" type="number" step="0.1"/>
                        </label>
                        <label>Technicien/Journée :
                            <input v-model="formulas.tech_day" type="number" />
                        </label>
                        <label>Technicien/Heure :
                            <input v-model="formulas.tech_hour" type="number" />
                        </label>
                        <label>Adhésion morale :
                            <input v-model="formulas.membership" type="number" />
                        </label>
                    </div>
                </div>
                <div class="types">
                    <h3>Types (Double click pour modifier)</h3>
                    <ul>
                        <li class="new">
                            <input
                                v-model="newTag"
                                @keyup.enter="addType"
                                placeholder="Ajouter un type..."
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
                            <button class="delete" @click="removeType(index)">x</button>
                        </li>
                    </ul>
                    <p v-if="tagMessage" class="error">{{ tagMessage }}</p>
                </div>
                <div class="admin">
                    <h2>Admin</h2>
                    <h3>Modifier le mot de passe</h3>
                    <label>Mot de passe actuel :
                        <input v-model="oldPassword" type="password"/>
                    </label>
                    <label>Nouveau mot de passe :
                        <input v-model="newPassword" type="password"/>
                    </label>
                    <label>Confirmer le nouveau mot de passe :
                        <input v-model="confirmNewPassword" type="password"/>
                    </label>
                </div>
                <div class="submit">
                    <button class="apply" @click="applyChanges">Appliquer</button>
                    <button class="cancel" @click="cancelChanges">Annuler</button>
                </div>
                <p v-if="applyMessage.message" :class="applyMessage.class">{{ applyMessage.message }}</p>
            </section>
        </div>
    </div>
</template>

<style scoped>
.params {
    max-width: 100%;
    display: grid;
    overflow-y: auto;
    overflow-x: hidden;
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

.submit {
    display: flex;
    gap: 1rem;
}

.apply {
    background-color: var(--success-background);
}

.apply:hover {
    background-color: var(--success);
}

.cancel {
    background-color: var(--disabled);
}

.cancel:hover {
    background-color: var(--border);
}

label {
    width: 100%;
    text-wrap: nowrap;
    font-size: 1rem;
}

.admin {
  display: flex;
  flex-direction: column;
  gap: 0.5rem;
  padding: 1rem;
}

.admin label {
  display: flex;
  align-items: center;
  justify-content: space-between;
  max-width: 28rem;
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

.formulas {
    display: flex;
    flex-wrap: wrap;
    gap: 2rem;
    margin: 0;
}

section div.pack {
    border-bottom: 0px;
    margin: 0;
    padding: 0;
}

.formulas label {
    display: grid;
    grid-template-columns: 1fr 1fr;
    max-width: 25rem;
    margin-top: 1rem;
}

.formulas p {
    margin-top: 0;
    font-size: 0.9rem;
}

p.success {
    color: var(--success);
}

p.error {
    color: var(--error);
}
</style>