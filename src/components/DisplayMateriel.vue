<script setup>
import { invoke } from '@tauri-apps/api/core';
import { computed, ref } from 'vue';

const props = defineProps(['item','setItem','setDocument']);
const emit = defineEmits(['item-change']);

const formulas = ref(null);
invoke('get_loc_formulas').then((data) => formulas.value = data);

const quantity = ref(1);
const duration = ref(1);

const lastOutings = ref([]);
invoke('get_factures_from_item', { itemId: props.item.id }).then((data) => lastOutings.value = data);

const followingRate = computed(() => {
    if(!props.item || !formulas.value) return 0;
    return props.item.contrib * formulas.value.contrib_following;
});

const renta = computed(() => {
    if(!props.item) return 0;
    return (props.item.benef*100)/props.item.valeur;
});

const updateValue = computed(() => {
    return props.item.total - quantity.value;
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

function setQuantityToMax() {
    quantity.value = props.item.total;
}

function formatDate(dateStr) {
    const date = new Date(dateStr);
    return new Intl.DateTimeFormat('fr-FR', {
        day: 'numeric',
        month: 'long',
        year: 'numeric'
    }).format(date);
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
            <h2>Calculer la contribution</h2>

            <div class="options">
                <div class="dispo-input">
                    <label for="quantity">Quantité</label>
                    <div class="quantity-field">
                        <input v-model="quantity" label="quantity" class="quantity-bar" type="number" min="0" :max="props.item.total" value="1" placeholder="Nombre d'objets"/>
                        <button class="quantity-button" @click="setQuantityToMax">Tout</button>
                    </div>
                    <p class="error" v-show="quantityError">{{ quantityError }}</p>
                </div>

                <div class="dispo-input">
                    <label for="time">Durée (Jours)</label>
                    <input v-model="duration" label="time" type="number" min="1" value="1" placeholder="Nombre de jours" />
                </div>

                <div>
                    <h3 class="contrib-total">Contribution totale : <span>{{ priceLoc.toFixed(2) }}€</span></h3>
                </div>
            </div>
        </section>
        <section class="latest" v-if="lastOutings.length > 0">
            <h2>Historique des sorties :</h2>
            <div v-for="out in lastOutings" class="outings" @click="setDocument({id: out.id, facture: true})">
                <h3>{{ out.nom }} : <span>{{ out.quantité }}</span></h3>
                <p>{{ formatDate(out.date) }} - {{ out.durée }} jours</p>
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

section:last-of-type {
    border-bottom: none;
}

.title {
    display: flex;
    align-items: center;
    justify-content: space-between;
}

.general h1 {
    font: inherit;
    font-weight: 700;
    margin: 0;
    font-size: 1.5rem;
}

h2 {
    margin: 0;
    margin-bottom: 1rem;
    font-size: 1.2rem;
    font-weight: 500;
}

.back {
    display: flex;
    justify-content: center;
    align-items: center;
    border-radius: 0.5rem;
    width: 2rem;
    height: 2rem;
    margin-right: 0.5rem;
}

.back:hover {
    cursor: pointer;
    background: var(--accent-hover);
}

.back {
    margin-left: auto;
}

h3 {
    margin-top: 1rem;
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
    justify-content: center;
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
}

.contrib-total {
    padding-bottom: 1rem;
}

.latest {
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
}

.outings {
    padding: 0.5rem;
    border: 1px solid var(--border);
    border-radius: 0.5rem;
}

.outings:hover {
    cursor: pointer;
    background-color: var(--surface-hover);
}

.outings h3 {
    margin: 0;
}
</style>