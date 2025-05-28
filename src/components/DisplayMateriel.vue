<script setup>
import { invoke } from '@tauri-apps/api/core';
import { computed, ref } from 'vue';

const props = defineProps(['item','setItem']);
const emit = defineEmits(['item-change']);

const formulas = ref(null);
invoke('get_loc_formulas').then((data) => {
    formulas.value = data
});

const following_rate = computed(() => {
    if(!props.item || !formulas.value) return 0;
    return props.item.contrib * formulas.value.contrib_following;
});

const usageFill = computed(() => {
    if(!props.item || props.item.total === 0) return 0;
    return Math.min(100, (props.item.dispo/props.item.total) * 100);
});

const usageColor = computed(() => {
    if (usageFill.value >= 70) return "#4caf50";
    if (usageFill.value >= 30) return "#ff9800";
    return "#f44336";
});

const mode = ref(null);
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

const quantity = ref(0);
const duration = ref(1);
const submit_error = ref(null); // TODO Feedback dans le champ de quantité

function setQuantityToMax() {
    quantity.value = mode.value == "Emprunter" ? props.item.dispo : props.item.total - props.item.dispo;
}

const update_value = computed(() => {
    submit_error.value = null;
    return mode.value == "Emprunter" ? props.item.dispo - quantity.value : props.item.dispo + quantity.value;
});

const quantity_error = computed(() => {
    if(quantity.value == "") {
        return " ";
    }
    if(!/^\d+$/.test(quantity.value)) {
        return "La quantité doit être un nombre positif";
    }
    if (update_value.value < 0) {
        return "Pas assez d'objets diponibles";
    }
    if (update_value.value > props.item.total) {
        return "Trop d'objets retournés";
    }

    return null;
});

const maxQuantity = computed(() => {
    return mode.value == "Emprunter" ? props.item.dispo : props.item.total - props.item.dispo;
});

const priceLoc = computed(() => {
    if(!duration.value || !quantity.value || !props.item || duration.value === 0) return 0;

    return quantity.value * (props.item.contrib + (duration.value - 1) * following_rate.value);
});



function updateDispo() {
    console.log(quantity_error.value);
    if(quantity_error.value) {
        submit_error.value = "Quantité non valide";
        return;
    }

    console.log("updating stonk");
    invoke('update_dispo', { value: update_value.value, id: props.item.id })
    .then(() => { 
        emit('item-change');
    })
    .catch((err) => { 
        submit_error.value = err; 
    });
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
                <div class="bar-wrapper">
                    <div class="bar-fill" :style="{ width: usageFill + '%', backgroundColor: usageColor }"></div>
                </div>
                <span class="value">{{ item.dispo }} / {{ item.total }}</span>
            </div>
        </section>
        <section class="stats">
            <p>Valeur de remplacement : {{ item.value }}€</p>
            <p>Nombre de sorties : {{ item.nb_sorties }}</p>
            <p>Estimation des revenus générés : {{ (item.nb_sorties * item.contrib).toFixed(2) }}€</p>
            <p>Taux de rentabilité estimée : {{ ((item.nb_sorties * item.contrib * 100)/item.value).toFixed(2) }}%</p>
        </section>
        <section class="price">
            <p>Contribution : <span>{{ item.contrib.toFixed(2) }}€</span> + {{ following_rate.toFixed(2) }}€ par jour supplémentaire</p>
        </section>
        <section class="dispo">
            <h2>Gérer cet élement</h2>

            <div class="select">
                <button :class="{selected : isSelected('Emprunter')}" @click="setMode('Emprunter')">Emprunter</button>
                <button :class="{selected : isSelected('Retourner')}" @click="setMode('Retourner')">Retourner</button>
            </div>
            <div class="options" v-if="mode">
                <div class="dispo-input">
                    <label for="quantity">Quantité</label>
                    <div class="quantity-field">
                        <input v-model="quantity" label="quantity" class="quantity-bar" type="number" min="0" :max="maxQuantity" placeholder="Nombre d'objets"/>
                        <button class="quantity-button" @click="setQuantityToMax">Tout</button>
                    </div>
                    <p class="error" v-show="quantity_error">{{ quantity_error }}</p>
                </div>

                <div class="dispo-input" v-if="isSelected('Emprunter')">
                    <label for="time">Durée (Jours)</label>
                    <input v-model="duration" label="time" type="number" min="1" placeholder="Nombre de jours" value="1"/>
                </div>

                <p v-if="isSelected('Emprunter') && priceLoc > 0" class="contrib-total">Contribution totale : <span>{{ priceLoc.toFixed(2) }}€</span></p>

                <button class="apply" :class="{disabled: quantity_error}" @click="updateDispo">Appliquer</button>
                <p v-show="submit_error" class="error">{{ submit_error }}</p>
            </div>
        </section>
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
    min-width: 18rem;
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
    margin-bottom: 1rem;
    max-width: 18rem;
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

button.selected {
    background-color: var(--accent-hover);
}

.contrib-total {
    padding-bottom: 1rem;
}

button.apply {
    width: 17rem;
}

button.apply.disabled {
    background-color: var(--disabled);
    cursor: default;
}

</style>