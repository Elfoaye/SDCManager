<script setup>
import { invoke } from '@tauri-apps/api/core';
import { ref, reactive, computed, watch } from 'vue';
import { useHasUploaded } from '../composables/hasUploadedStore';
const { setHasUploaded } = useHasUploaded();

const props = defineProps(['item','setItem', 'setItemRefresh', 'create']);
const emit = defineEmits(['item-change']);

const types = ref([]);
invoke('get_materiel_types').then((data) => types.value = data);
const formulas = ref(null);
invoke('get_loc_formulas').then((data) => formulas.value = data);

const tempItem = ref({
    id: 0,
    nom: '',
    item_type: '',
    total: '',
    valeur: '',
    contrib: '',
    nb_sorties: 0,
    benef: 0
});

const confirm = ref(null);
const isLoading = ref(false);

const submitError = ref(null);
const requiredFields = reactive({
  nom: false,
  item_type: false,
  total: false,
  valeur: false,
  contrib: false,
  nb_sorties: false,
  benef: false
});

const badFields = computed(() => {
    return Object.values(requiredFields).some(val => val);
});

function verifField(field) {
    const value = tempItem.value[field];

    if(['nb_sorties', 'benef'].includes(field)) {
        if(value && value < 0) {
            requiredFields[field] = true;
            return false;
        }
    } else if (!value ||
        (typeof value === 'string' && value.trim() === '' )||
        (!['nom', 'item_type'].includes(field) && (typeof value !== 'number' || value < 0)) ||
        (field === 'item_type' && !types.value.includes(value)) ||
        (field === 'total' && value <= 0)) {
        
        requiredFields[field] = true;
        return false;
    }
    
    requiredFields[field] = false;
    return true;
}

function verifFields() {
    let ok = true;

    Object.keys(requiredFields).forEach(field => {
        if (!verifField(field)) {
            ok = false;
        }
    });

    return ok;
}

function confirmDelete() {
    if(!props.item) return;

    confirm.value = 'delete';
}

function confirmApplyChanges() {
    isLoading.value = true;

    if(!verifFields()) {
        submitError.value = "Certains champs sont invalides";
        isLoading.value = false;
        return;
    }
    
    confirm.value = 'change';
}

function confirmCancel() {
    confirm.value = null;
    isLoading.value = false;
}

async function deleteItem() {
    confirmCancel();

    try {
        await invoke('delete_item', { id: props.item.id }).then();
        props.setItemRefresh(null);
        setHasUploaded(false);  
    } catch (err) {
        submitError.value = err;
        console.error(err);
    } finally {
        isLoading.value = false;
    }
}

function applyItemChanges() {
    confirmCancel();
    try {
        invoke('update_item', { item: tempItem.value });
        emit('item-change');
        setHasUploaded(false);  
    } catch (err) {
        submitError.value = err;
        console.error(err);
    } finally {
        isLoading.value = false;
    }
}

async function addItem() {
    isLoading.value = true;
    if(!verifFields()) {
        submitError.value = "Certains champs sont invalides";
        isLoading.value = false;
        return;
    }

    let id;
    try {
        id = await invoke('add_item', { item: tempItem.value });
        props.setItemRefresh(id);
        setHasUploaded(false);  
    } catch (err) {
        if(err === 'UNIQUE constraint failed: Materiel.nom') {
            submitError.value = 'Un objet avec ce nom existe déjà';
        } else {
            submitError.value = err;
        }
        console.error(err);
    } finally {
        isLoading.value = false;
    }
}

function setContribFromValue() {
    const value = tempItem.value.valeur;
    if(!verifField('valeur') || formulas.value === null) return;

    tempItem.value.contrib = Number((value * formulas.value.contrib_first_day).toFixed(2));
    verifField('contrib');
}

watch(
    () => props.item, (newItem) => {
        if(newItem && !props.create) {
            tempItem.value = { ...newItem };
        }
    }, 
    { immediate: true }
);
</script>

<template>
    <div class="itemCard" :class="{ new: create }">
        <div v-if="confirm" class="confirm">
            <div class="pop-up">
                <p v-if="confirm === 'delete'">Êtes-vous sûr de vouloir supprimer <span>{{ item.nom }}</span> ?</p>
                <p v-else>Appliquer les modifications sur <span>{{ item.nom }}</span> ?</p>

                <div class="confirm-buttons">
                    <button v-if="confirm === 'delete'" @click="deleteItem" class="delete-confirm" >Supprimer</button>
                    <button v-else @click="applyItemChanges" class="change">Modifier</button>
                    <button @click="confirmCancel" class="cancel">Annuler</button>
                </div>
            </div>
        </div>

        <div class="title">
            <h1 v-if="create">Nouvel objet</h1>
            <h1 v-else>Modifier l'objet</h1>
            
            <button class="back" @click="setItem(null)">X</button>
        </div>
        
        <section class="general form-grid">
            <label><h2>Nom :</h2> <input v-model="tempItem.nom" @input="verifField('nom')" :class="{ error: requiredFields.nom }" placeholder="Nom de l'objet..."/></label>
            <label><h3>Catégorie :</h3>
                <select v-model="tempItem.item_type" @change="verifField('item_type')" :class="{ error: requiredFields.item_type }" placeholder="Catégorie de l'objet...">
                    <option disabled value="">-- Choisir un type --</option>
                    <option v-for="type in types" :value="type">{{ type }}</option>
                </select>
            </label>
        </section>
        <section class="stats form-grid">
            <label>Total : <input v-model="tempItem.total" @input="verifField('total')" :class="{ error: requiredFields.total }" type="number" min="0" placeholder="Quantitée totale d'objets..."/></label>
            <label>Valeur (€) : <input v-model="tempItem.valeur" @input="setContribFromValue()" :class="{ error: requiredFields.valeur }" type="number" step="0.01" min="0" placeholder="Valeur de remplacement..."/> </label>
            <label>Contribution (€) : <input v-model="tempItem.contrib" @input="verifField('contrib')" :class="{ error: requiredFields.contrib }" type="number" step="0.01" min="0" placeholder="Contribution par jour..."/> </label>
        </section>
        <section class="advanced form-grid">
            <label>Nombre de sorties : <input v-model="tempItem.nb_sorties" @input="verifField('nb_sorties')" :class="{ error: requiredFields.nb_sorties }" type="number" min="0" placeholder="0"/></label>
            <label>Bénéfices : <input v-model="tempItem.benef" @input="verifField('benef')" :class="{ error: requiredFields.benef }" type="number" step="0.01" min="0" placeholder="0"/></label>
        </section>

        
        <button 
            v-if="create"
            class="apply add-item" 
            :class="{ 'disabled': badFields || isLoading}" 
            @click="addItem"
        >
            <template v-if="isLoading">
                <span class="spinner"></span>
            </template>
            <template v-else>
                + Ajouter l'objet
            </template>
        </button>
        <div class="submit-buttons" v-else>
            <button 
                class="apply" 
                :class="{ 'disabled': badFields || isLoading}" 
                @click="confirmApplyChanges"
            >
                <template v-if="isLoading">
                    <span class="spinner"></span>
                </template>
                <template v-else>
                    &#10003; Appliquer les modifications
                </template>
            </button>
            <button @click="confirmDelete" class="delete">Supprimer l'objet</button>
        </div>
        <p v-if="submitError" class="error">{{ submitError }}</p>
    </div>
</template>

<style scoped>
.itemCard {
    border: 2px solid var(--warning);
}

.itemCard.new {
    border: 2px solid var(--success);
}

.confirm {
    position: absolute;
    top: 0;
    left: 0;
    right: 0;
    bottom: 0;
    display: flex;
    justify-content: center;
    align-items: center;
    background-color: rgba(0, 0, 0, 0.5);
    z-index: 1000;
}

.pop-up {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    max-height: 5rem;
    background-color: var(--background-alt);
    border: 1px solid var(--border-accent);
    border-radius: 0.5rem;
    padding: 1rem;
}

.change {
    background-color: var(--warning);
}

.cancel {
    background-color: var(--disabled);
}

section {
    display: flex;
    flex-direction: column;
    padding-top: 0.5rem;
    padding-bottom: 0.5rem;
    border-bottom: 1px solid var(--border);
}

.form-grid {
    display: grid;
    grid-template-columns: 10rem 1fr;
    gap: 0.5rem 1rem;
    align-items: center;
}

.form-grid label {
    display: contents;
}

.form-grid input,
.form-grid select {
    width: 100%;
    max-width: 10rem;
    padding: 0.5rem;
    box-sizing: border-box;
}

.general select {
    padding: 0.5rem;
    color: var(--text);
    background-color: var(--background-alt);
    border: 1px solid var(--border);
    border-radius: 0.3rem;
}

.general h2 {
    margin: 0;
    font-size: 1.2rem;
}
.general h3 {
    margin: 0;
    font-size: 1.rem;
    font-weight: 500;
}

.back {
    display: flex;
    justify-content: center;
    align-items: center;
    background: var(--accent);
    border: 1px solid var(--border);
    border-radius: 0.5rem;
    width: 2rem;
    height: 2rem;
}

.back:hover {
    cursor: pointer;
    background: var(--accent-hover);
}

.back {
    margin-left: auto;
}

input {
    max-width: 16rem;
    padding: 0.5rem;
    color: var(--text);
    background-color: var(--background-alt);
    border: 1px solid var(--border);
    border-radius: 0.3rem;
}

.error {
    color: var(--error);
}

input.error,
.general select.error  {
    box-shadow: inset 0 0 5px var(--error);
    color: var(--text);
}

button {
    height: 100%;
    padding: 1rem;
    margin-right: 1rem;
    width: 8rem;
    color: var(--text);
    background-color: var(--accent);
    border: 1px solid var(--border);
    border-radius: 0.5rem;

    transition: all 0.2s;
}

button:hover {
    background-color: var(--surface-selected);
    cursor: pointer;
    
    transition: all 0.2s;
}

.submit-buttons {
    display: flex;
    margin-bottom: 0.5rem;
}

button.apply {
    margin-top: 1rem;
    background-color: var(--warning);
    width: 17rem;
    font-weight: 600;
}

button.apply.add-item {
    background-color: var(--success);
}

button.apply.disabled {
    background-color: var(--disabled);
    cursor: default;
    font-weight: 400;
}

button.delete {
    margin-top: 1rem;
    width: 17rem;
    font-weight: 600;
    background-color: var(--background-error);
}

button.delete-confirm {
    width: 8rem;
    background-color: var(--background-error);
}

button.delete:hover {
    background-color: var(--error);
}

.confirm-buttons {
    margin-top: 1rem;
}

.spinner {
  display: inline-block;
  width: 1em;
  height: 1em;
  border: 2px solid var(--text);
  border-top: 2px solid transparent;
  border-radius: 50%;
  animation: spin 0.6s linear infinite;
  margin-right: 0.5em;
}

@keyframes spin {
  to {
    transform: rotate(360deg);
  }
}
</style>