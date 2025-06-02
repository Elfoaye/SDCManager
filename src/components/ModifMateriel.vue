<script setup>
import { invoke } from '@tauri-apps/api/core';
import { computed, ref, watch } from 'vue';

const props = defineProps(['item','setItem','create']);
const emit = defineEmits(['item-change']);

const types = ref([]);
invoke('get_materiel_types').then((data) => types.value = data);
const formulas = ref(null);
invoke('get_loc_formulas').then((data) => formulas.value = data);

const tempItem = ref({
    nom: '',
    item_type: '',
    total: '',
    dispo: '',
    value: '',
    contrib: '',
    nb_sorties: '',
    benef: ''
});

const isLoading = ref(false);

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
        <div class="title">
            <h1 v-if="create">Nouvel objet</h1>
            <div v-else class="title-text">
                <h1>Modifier l'objet</h1>
                <button class="delete">Supprimer l'objet</button>
            </div>
            
            <button class="back" @click="setItem(null)">X</button>
        </div>
        
        <section class="general">
            <div class="title">
                <label>Nom : 
                    <input v-model="tempItem.nom" placeholder="Nom de l'objet..."/>
                </label>
            </div>
            <label>Catégorie : 
                <select v-model="tempItem.item_type" placeholder="Catégorie de l'objet...">
                    <option v-for="type in types">{{ type }}</option>
                </select>
            </label>
        </section>
        <section class="stats">
            <label>Disponible : <input v-model="tempItem.dispo" placeholder="Quantitée d'objets disponibles..."/></label>
            <label>Total : <input v-model="tempItem.total" placeholder="Quantitée totale d'objets..."/></label>
            <label>Valeur : <input v-model="tempItem.value" placeholder="Valeur de remplacement..."/> €</label>
            <label>Contribution : <input v-model="tempItem.contrib" placeholder="Contribution par jour..."/> €</label>
        </section>
        <section class="advanced">
            <label>Nobre de sorties : <input v-model="tempItem.nb_sorties" placeholder="Nombre total de sorties..."/></label>
            <label>Bénéfices : <input v-model="tempItem.benef" placeholder="Bénéfices totaux..."/></label>
        </section>

        <button>Appercu des modifications</button>

        <button 
            class="apply" 
            :class="{disabled: quantityError || isLoading}" 
            @click="updateDispo"
        >
            <template v-if="isLoading">
                <span class="spinner"></span>
            </template>
            <template v-else>
                &#10003; Appliquer les modifications
            </template>
        </button>
    </div>
</template>

<style scoped>
.itemCard {
    flex-grow: 1;
    display: flex;
    flex-direction: column;
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
    background-color: var(--background-error);
    color: var(--background-alt);
}

button.delete:hover {
    background-color: var(--error);
}

button.apply {
    width: 17rem;
    font-weight: 600;
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