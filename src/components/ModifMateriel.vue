<script setup>
import { invoke } from '@tauri-apps/api/core';
import { ref, reactive, computed, watch } from 'vue';

const props = defineProps(['item','setItem','create']);
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
    dispo: 0,
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
  dispo: false,
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

    if(['nb_sorties', 'benef', 'dispo'].includes(field)) {
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
        isLoading = false;
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
    } catch (err) {
        console.error(err);
    } finally {
        isLoading.value = false;
        props.setItem(null);
        emit('item-change');
    }
}

function applyItemChanges() {
    confirmCancel();
    try {
        invoke('update_item', { item: tempItem.value });
    } catch (err) {
        console.error(err);
    } finally {
        isLoading.value = false;
        emit('item-change');
    }
}

function addItem() {
    isLoading.value = true;
    if(!verifFields()) {
        submitError.value = "Certains champs sont invalides";
        isLoading.value = false;
        return;
    }

    try {
        invoke('add_item', { item: tempItem.value });
    } catch (err) {
        console.error(err);
    } finally {
        isLoading.value = false;
        props.setItem(null);
        emit('item-change');
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
        else if(!props.create) {
            tempItem.value.id = newItem.id;
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
                    <button v-if="confirm === 'delete'" @click="deleteItem" class="delete" >Supprimer</button>
                    <button v-else @click="applyItemChanges" class="change">Modifier</button>
                    <button @click="confirmCancel" class="cancel">Annuler</button>
                </div>
            </div>
        </div>

        <div class="title">
            <h1 v-if="create">Nouvel objet</h1>
            <div v-else class="title-text">
                <h1>Modifier l'objet</h1>
                <button @click="confirmDelete" class="delete">Supprimer l'objet</button>
            </div>
            
            <button class="back" @click="setItem(null)">X</button>
        </div>
        
        <section class="general">
            <div class="title">
                <label>Nom : 
                    <input v-model="tempItem.nom" @input="verifField('nom')" :class="{ error: requiredFields.nom }" placeholder="Nom de l'objet..."/>
                </label>
            </div>
            <label>Catégorie : 
                <select v-model="tempItem.item_type" @input="verifField('item_type')" :class="{ error: requiredFields.item_type }" placeholder="Catégorie de l'objet...">
                    <option v-for="type in types">{{ type }}</option>
                </select>
            </label>
        </section>
        <section class="stats">
            <label v-if="!create">Disponible : <input v-model="tempItem.dispo" @input="verifField('dispo')" :class="{ error: requiredFields.dispo }" type="number" min="0" placeholder="Quantitée d'objets disponibles..."/></label>
            <label>Total : <input v-model="tempItem.total" @input="verifField('total')" :class="{ error: requiredFields.total }" type="number" min="0" placeholder="Quantitée totale d'objets..."/></label>
            <label>Valeur : <input v-model="tempItem.valeur" @input="setContribFromValue()" :class="{ error: requiredFields.valeur }" type="number" step="0.01" min="0" placeholder="Valeur de remplacement..."/> €</label>
            <label>Contribution : <input v-model="tempItem.contrib" @input="verifField('contrib')" :class="{ error: requiredFields.contrib }" type="number" step="0.01" min="0" placeholder="Contribution par jour..."/> €</label>
        </section>
        <section class="advanced">
            <label>Nombre de sorties : <input v-model="tempItem.nb_sorties" @input="verifField('nb_sorties')" :class="{ error: requiredFields.nb_sorties }" type="number" min="0" placeholder="0"/></label>
            <label>Bénéfices : <input v-model="tempItem.benef" @input="verifField('benef')" :class="{ error: requiredFields.benef }" type="number" step="0.01" min="0" placeholder="0"/></label>
        </section>

        <!-- <button>Appercu des modifications</button> -->

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
        <button 
            v-else
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
        <p v-if="badFields" class="error">{{ submitError }}</p>
    </div>
</template>

<style scoped>
.itemCard {
    flex-grow: 1;
    display: flex;
    flex-direction: column;
    position: relative;
    height: fit-content;
    min-height: 20rem;
    max-height: 90%;
    min-width: 20rem;
    max-width: 50rem;
    margin: 0.5rem;
    padding: 1em;
    overflow: auto;
    border: 2px solid var(--warning);
    border-radius: 0.5rem;
}

.itemCard.new {
    border: 2px solid var(--success);
}

.itemCard p {
    margin: 0;
}

.itemCard span {
    font-weight: 500;
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
    padding: 1em;
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

.title {
    display: flex;
}

.title-text {
    display: flex;
    align-items: center;
    gap: 1rem;
}

.title h1 {
    margin: 0;
}

.general h1 {
    margin: 0;
    font-size: 1.5rem;
}

.general h2, .dispo h2 {
    margin: 0;
    margin-bottom: 1rem;
    font-size: 1.2rem;
    font-weight: 500;
}

.general select {
    max-width: 16rem;
    padding: 0.5rem;
    color: var(--text);
    background-color: var(--background-alt);
    border: 1px solid var(--border);
    border-radius: 0.3rem;
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

button.delete {
    max-height: 4rem;
    background-color: var(--background-error);
    color: var(--background-alt);
}

button.delete:hover {
    background-color: var(--error);
}

button.apply {
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