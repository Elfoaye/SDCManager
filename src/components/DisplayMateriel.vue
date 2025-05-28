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
const quantity = ref(0);
const duration = ref(1);
const quantity_error = ref(null);
const submit_message = ref(null);
const submit_error = ref(null);

const maxQuantity = computed(() => {
    return mode.value == "Emprunter" ? props.item.dispo : props.item.total - props.item.dispo;
});

const priceLoc = computed(() => {
    if(!duration.value || !quantity.value || !props.item || duration.value === 0) return 0;

    return quantity.value * (props.item.contrib + (duration.value - 1) * following_rate.value);
});

function updateDispo() {
    const new_value = mode.value == "Emprunter" ? props.item.dispo - quantity.value : props.item.dispo + quantity.value;
    
    if (new_value < 0) {
        quantity_error.value = "Pas assez d'objets diponibles";
        return;
    }
    if (new_value > props.item.total) {
        quantity_error.value = "Trop d'objets retournés";
        return;
    }

    quantity_error.value = null;

    invoke('update_dispo', { value: new_value, id: props.item.id })
    .then((value) => { 
        submit_message.value = value; 
        emit('item-change');
    })
    .catch((err) => { submit_error.value = err; });
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
                <button class="item-borrow" @click="setMode('Emprunter')">Emprunter</button>
                <button class="item-return" @click="setMode('Retourner')">Retourner</button>
            </div>
            <div class="options" v-if="mode">
                <h2>{{ mode }}</h2>

                <div class="dispo-input">
                    <label for="quantity">Quantité</label>
                    <input v-model="quantity" label="quantity" type="number" min="0" :max="maxQuantity" placeholder="Nombre d'objets"/>
                    <p class="error" v-show="quantity_error">{{ quantity_error }}</p>
                </div>

                <div class="dispo-input" v-if="mode === 'Emprunter'">
                    <label for="time">Durée</label>
                    <input v-model="duration" label="time" type="number" min="0" placeholder="Nombre de jours" value="1"/>
                </div>

                <p v-if="mode === 'Emprunter' && priceLoc > 0">Contribution totale : {{ priceLoc.toFixed(2) }}€</p>

                <button @click="updateDispo">Appliquer</button>
                <p v-show="submit_message">{{ submit_message }}</p>
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
    height: auto;
    min-width: 18rem;
    max-width: 50rem;
    margin: 0.5rem;
    padding: 1em;
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

.title button {
    background: var(--accent);
    border: 1px solid var(--border);
    border-radius: 0.5rem;
    width: 2rem;
    height: 2rem;
}

.title button:hover {
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
}

.options {
    display: flex;
    flex-direction: column;
}

.dispo-input { 
    display: flex;
    flex-direction: column;
    margin-bottom: 1rem;
}

input {
    width: 100%;
    max-width: 15rem;
    padding: 0.5rem;
}

.error {
    color: var(--error);
}

</style>