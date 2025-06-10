<script setup>
import { invoke } from '@tauri-apps/api/core';
import { computed, ref } from 'vue';

const props = defineProps(['item','setItem']);
const emit = defineEmits(['item-change']);

const formulas = ref(null);
invoke('get_loc_formulas').then((data) => formulas.value = data);

const mode = ref(null);
const quantity = ref(1);
const duration = ref(1);
const isFree = ref(false);
const isLoading = ref(false);

const followingRate = computed(() => {
    if(!props.item || !formulas.value) return 0;
    return props.item.contrib * formulas.value.contrib_following;
});

const usageFill = computed(() => {
    if(!props.item || props.item.total === 0) return 0;
    return Math.min(100, (props.item.dispo/props.item.total) * 100);
});

const usageColor = computed(() => {
    if (usageFill.value == 100) return "#4caf50";
    if (usageFill.value > 50) return "#F9DD00";
    return "#ff9800";
});

const renta = computed(() => {
    if(!props.item) return 0;
    return (props.item.benef*100)/props.item.valeur;
});

const updateValue = computed(() => {
    return mode.value == "Emprunter" ? props.item.dispo - quantity.value : props.item.dispo + quantity.value;
});

const maxQuantity = computed(() => {
    return mode.value == "Emprunter" ? props.item.dispo : props.item.total - props.item.dispo;
});

const quantityError = computed(() => {
    if(quantity.value == "") {
        return " ";
    }
    if(!/^\d+$/.test(quantity.value)) {
        return "La quantité doit être un nombre positif";
    }
    if (updateValue.value < 0) {
        return "Pas assez d'objets diponibles";
    }
    if (updateValue.value > props.item.total) {
        return "Trop d'objets retournés";
    }

    return null;
});

const priceLoc = computed(() => {
    if(!duration.value || !quantity.value || !props.item || duration.value === 0) return 0;

    return quantity.value * (props.item.contrib + (duration.value - 1) * followingRate.value);
});

function setMode(new_mode) {
    if(mode.value === new_mode) {
        mode.value = null;
        return
    }
    mode.value = new_mode;
}

function isSelected(check_mode) {
    return mode.value === check_mode;
}

function setQuantityToMax() {
    quantity.value = mode.value == "Emprunter" ? props.item.dispo : props.item.total - props.item.dispo;
}

async function updateDispo() {
    if(quantityError.value) {
        return;
    }

    isLoading.value = true
    const profit = isFree.value || mode.value !== 'Emprunter' ? 0 : priceLoc.value;

    try {
        await invoke('update_dispo', { valeur: updateValue.value, old: props.item.dispo, benef: profit, id: props.item.id });
        emit('item-change');
    } catch (err) {
        console.error(err);
    } finally {
        isLoading.value = false;
    }
}
</script>

<template>
    <div class="itemCard">
        <section class="general">
            <div class="title">
                <h1>{{ item.nom }}</h1>
                <button class="back" @click="setItem(null)">X</button>
            </div>
            <h2>{{ item.item_type }}</h2>
            <div class="availability">
                <span v-if="item.dispo<=0" class="label">&#9888;</span>
                <span class="label">Disponible :</span>
                <div class="bar-wrapper" :class="{ empty: usageFill == 0}">
                    <div class="bar-fill" :style="{ width: usageFill + '%', backgroundColor: usageColor }"></div>
                </div>
                <span class="value">{{ item.dispo }} / {{ item.total }}</span>
            </div>
        </section>
        <section class="stats">
            <p>Valeur de remplacement : <span>{{ item.valeur }}€</span>/Objet (Total : {{ item.valeur * item.total }}€)</p>
            <p>Nombre de sorties : <span>{{ item.nb_sorties }}</span></p>
            <p>Revenus générés : <span>{{ (item.benef).toFixed(2) }}€</span></p>
            <p>Taux de rentabilité : <span>{{ (renta/item.total).toFixed(2) }}%</span> ({{ renta.toFixed(2) }}% d'un objet)</p>
        </section>
        <section class="price">
            <p>Contribution : <span>{{ item.contrib.toFixed(2) }}€</span> + {{ followingRate.toFixed(2) }}€ par jour supplémentaire</p>
        </section>
        <section class="dispo">
            <h2>Gérer cet élement</h2>

            <div class="select">
                <button :class="{selected : isSelected('Emprunter')}" @click="setMode('Emprunter')">
                    <svg width="24" height="24" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
                        <path d="M6.5 8.5L4.78246 9.64502C4.31358 9.95761 4.07914 10.1139 3.90942 10.3204C3.75917 10.5032 3.64643 10.7139 3.57767 10.9403C3.5 11.196 3.5 11.4778 3.5 12.0413V15.62C3.5 16.6281 3.5 17.1321 3.69619 17.5172C3.86876 17.8559 4.14413 18.1312 4.48282 18.3038C4.86786 18.5 5.37191 18.5 6.38 18.5H17.62C18.6281 18.5 19.1321 18.5 19.5172 18.3038C19.8559 18.1312 20.1312 17.8559 20.3038 17.5172C20.5 17.1321 20.5 16.6281 20.5 15.62V12.0413C20.5 11.4778 20.5 11.196 20.4223 10.9403C20.3536 10.7139 20.2408 10.5032 20.0906 10.3204C19.9209 10.1139 19.6864 9.95761 19.2175 9.64502L17.5 8.5" stroke="currentColor" stroke-linecap="round"/>
                        <path d="M20.5 12.5H17.9415C17.2458 12.5 16.8979 12.5 16.636 12.6888C16.3742 12.8775 16.2642 13.2075 16.0442 13.8675L15.9558 14.1325C15.7358 14.7925 15.6258 15.1225 15.364 15.3112C15.1021 15.5 14.7542 15.5 14.0585 15.5H9.94152C9.2458 15.5 8.89794 15.5 8.63605 15.3112C8.37416 15.1225 8.26416 14.7925 8.04415 14.1325L7.95585 13.8675C7.73584 13.2075 7.62584 12.8775 7.36395 12.6888C7.10206 12.5 6.7542 12.5 6.05848 12.5H3.5" stroke="currentColor"/>
                        <path d="M9.5 5.5L12 3M12 3L14.5 5.5M12 3L12 10" stroke="currentColor"/>
                    </svg>
                    Emprunter
                </button>
                <button :class="{selected : isSelected('Retourner')}" @click="setMode('Retourner')">
                    <svg width="24" height="24" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
                        <path d="M20.5 12.5H17.9415C17.2458 12.5 16.8979 12.5 16.636 12.6888C16.3742 12.8775 16.2642 13.2075 16.0442 13.8675L15.9558 14.1325C15.7358 14.7925 15.6258 15.1225 15.364 15.3112C15.1021 15.5 14.7542 15.5 14.0585 15.5H9.94152C9.2458 15.5 8.89794 15.5 8.63605 15.3112C8.37416 15.1225 8.26416 14.7925 8.04415 14.1325L7.95585 13.8675C7.73584 13.2075 7.62584 12.8775 7.36395 12.6888C7.10206 12.5 6.7542 12.5 6.05848 12.5H3.5" stroke="currentColor"/>
                        <path d="M9.5 9.5L12 12M12 12L14.5 9.5M12 12L12 5" stroke="currentColor"/>
                        <path d="M6.5 8.5L4.78246 9.64502C4.31358 9.95761 4.07914 10.1139 3.90942 10.3204C3.75917 10.5032 3.64643 10.7139 3.57767 10.9403C3.5 11.196 3.5 11.4778 3.5 12.0413V15.62C3.5 16.6281 3.5 17.1321 3.69619 17.5172C3.86876 17.8559 4.14413 18.1312 4.48282 18.3038C4.86786 18.5 5.37191 18.5 6.38 18.5H17.62C18.6281 18.5 19.1321 18.5 19.5172 18.3038C19.8559 18.1312 20.1312 17.8559 20.3038 17.5172C20.5 17.1321 20.5 16.6281 20.5 15.62V12.0413C20.5 11.4778 20.5 11.196 20.4223 10.9403C20.3536 10.7139 20.2408 10.5032 20.0906 10.3204C19.9209 10.1139 19.6864 9.95761 19.2175 9.64502L17.5 8.5" stroke="currentColor" stroke-linecap="round"/>
                    </svg>
                    Retourner
                </button>
            </div>
            <div class="options" v-if="mode">
                <div class="dispo-input">
                    <label for="quantity">Quantité</label>
                    <div class="quantity-field">
                        <input v-model="quantity" label="quantity" class="quantity-bar" type="number" min="0" :max="maxQuantity" value="1" placeholder="Nombre d'objets"/>
                        <button class="quantity-button" @click="setQuantityToMax">Tout</button>
                    </div>
                    <p class="error" v-show="quantityError">{{ quantityError }}</p>
                </div>

                <div class="dispo-input" v-if="isSelected('Emprunter')">
                    <label for="time">Durée (Jours)</label>
                    <input v-model="duration" label="time" type="number" min="1" value="1" placeholder="Nombre de jours" />
                </div>

                <div v-if="isSelected('Emprunter')">
                    <label class="check"><input v-model="isFree" type="checkbox"/>Prêt (la contribution ne s'applique pas)</label>

                    <p v-if="!isFree && priceLoc > 0" class="contrib-total">Contribution totale : <span>{{ priceLoc.toFixed(2) }}€</span></p>
                </div>

                <button 
                    class="apply" 
                    :class="{disabled: quantityError || isLoading}" 
                    @click="updateDispo"
                >
                    <template v-if="isLoading">
                        <span class="spinner"></span>
                    </template>
                    <template v-else>
                        &#10003; Appliquer
                    </template>
                </button>
            </div>
        </section>
    </div>
</template>

<style scoped>
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
    font: inherit;
    font-weight: 700;
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

.availability {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    width: 100%;
    max-width: 30rem;
}

.bar-wrapper {
    flex-grow: 1;
    height: 1rem;
    background-color: var(--background-alt);
    border-radius: 0.5rem;
    overflow: hidden;
    position: relative;
    outline: 1px solid var(--border);
}

.bar-wrapper.empty {
    outline-color: var(--error);
}

.bar-fill {
    height: 100%;
    transition: width 0.3s ease;
}

.stats {
    flex-wrap: wrap;
    gap: 0 2rem;
}

.dispo {
    display: flex;
    flex-direction: column;
    border: 0;
}

.options {
    display: flex;
    flex-direction: column;
    width: 100%;
}

input {
    max-width: 16rem;
    padding: 0.5rem;
    color: var(--text);
    background-color: var(--background-alt);
    border: 1px solid var(--border);
    border-radius: 0.3rem;
}

.dispo-input { 
    display: flex;
    flex-direction: column;
    margin-bottom: 1rem;
}

.quantity-field {
    display: flex;
    width: 100%;
    max-width: 18rem;
}

.quantity-bar {
    flex-grow: 1;
    border-top-right-radius: 0;
    border-bottom-right-radius: 0;
}

.quantity-button{
    height: 100%;
    padding: 0.5rem;
    width: 4rem;
    background-color: var(--accent);
    color: var(--text);
    border: 1px solid var(--border);
    border-left: 0;
    border-top-right-radius: 0.3rem;
    border-bottom-right-radius: 0.3rem;
    border-top-left-radius: 0;
    border-bottom-left-radius: 0;

    transition: all 0.2s;
}

.quantity-button:hover {
    background-color: var(--accent-hover);
    cursor: pointer;
    
    transition: all 0.2s;
}

.error {
    color: var(--error);
}

.select {
    display: flex;
    flex-wrap: nowrap;
    margin-bottom: 1rem;
    max-width: 18rem;
}

button {
    margin-right: 1rem;
    display: flex;
    align-items: center;
    gap: 0.3rem;

    transition: all 0.2s;
}

button:hover {
    background-color: var(--surface-selected);
    cursor: pointer;
    
    transition: all 0.2s;
}

button.selected {
    background-color: var(--accent-hover);
}

.contrib-total {
    padding-bottom: 1rem;
}

.check {
    margin-bottom: 1rem;
    cursor: pointer;
    max-width: fit-content;
    align-items: baseline;
}

.check input {
    margin-right: 0.5rem;
    cursor: pointer;
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