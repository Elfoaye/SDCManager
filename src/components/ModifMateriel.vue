<script setup>
import { invoke } from '@tauri-apps/api/core';
import { computed, ref } from 'vue';

const props = defineProps(['item','setItem']);
const emit = defineEmits(['item-change']);

const formulas = ref(null);
invoke('get_loc_formulas').then((data) => formulas.value = data);

const isLoading = ref(false);
</script>

<template>
    <div class="itemCard">
        <section class="general">
            <div class="title">
                <h1>{{ item.nom }}</h1>
                <button class="back" @click="setItem(null)">X</button>
            </div>
            <h2>{{ item.type }}</h2>
            <div class="availability">
                <p>Nombre total</p>
            </div>
        </section>
        <section class="stats">
            <p>Valeur de remplacement</p>
            <p>Contribution</p>
        </section>

        <button>Supprimer</button>

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

<style scopped>
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
    border: 1px solid var(--border-accent);
    border-radius: 0.5rem;
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