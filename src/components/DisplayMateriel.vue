<script setup>
import { invoke } from '@tauri-apps/api/core';
import { computed, nextTick, ref, watch } from 'vue';
// import { Chart, BarController, BarElement, CategoryScale, LinearScale, Tooltip, Legend } from 'chart.js';
// Chart.register(BarController, BarElement, CategoryScale, LinearScale, Tooltip, Legend);

const props = defineProps(['item','setItem']);

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

// const nbDays = ref(7);
// const contribLabels = computed(() =>{
//     return Array.from({ length: nbDays.value }, (_, i) => `Jour ${i + 1}`);
// });
// const contribData = computed(() => {
//     return Array.from({ length: nbDays.value }, (_, i) => (props.item.contrib + (i * following_rate.value)).toFixed(2));
// });
// const chartRef  = ref(null);
// const chartInstance = ref(null);

// watch([chartRef, contribData], () => {
//     if (!chartRef.value || contribData.value.length === 0) return;

//     if(chartInstance.value) {
//         chartInstance.value.data.datasets[0].data = contribData.value;
//         chartInstance.value.data.labels = contribLabels.value;
//         chartInstance.value.update();
//     }

//     chartInstance.value = new Chart(chartRef.value, {
//         type: 'bar',
//         data: {
//             labels: contribLabels.value,
//             datasets: [{
//                 label: 'Contribution (€)',
//                 data: contribData.value,
//             }]
//         },
//         options: {
//             responsive: true,
//             maintainAspectRatio: false,
//             scales: {
//                 y: {
//                     beginAtZero: true
//                 }
//             }
//         }
//     });
// });
</script>

<template>
    <div class="itemCard">
        <div class="general">
            <div class="title">
                <h1>{{ item.nom }}</h1>
                <button class="back" @click="setItem(null)">X</button>
            </div>
            <h2>{{ item.item_type }}</h2>
            <div class="availability">
                <span class="label">Disponible :</span>
                <div class="bar-wrapper">
                    <div class="bar-fill" :style="{ width: usageFill + '%', backgroundColor: usageColor }"></div>
                </div>
                <span class="value">{{ item.dispo }} / {{ item.total }}</span>
            </div>
        </div>
        <div class="stats">
            <p>Valeur de remplacement : {{ item.value }}€</p>
            <p>Nombre de sorties : {{ item.nb_sorties }}</p>
            <p>Estimation des revenus générés : {{ (item.nb_sorties * item.contrib).toFixed(2) }}€</p>
            <p>Taux de rentabilité estimée : {{ ((item.nb_sorties * item.contrib * 100)/item.value).toFixed(2) }}%</p>
        </div>
        <div class="price">
            <p>Contribution : <span>{{ item.contrib.toFixed(2) }}€</span> + {{ following_rate.toFixed(2) }}€ par jour supplémentaire</p>
            <!-- <canvas ref="chartRef" style="width: 100%; height: 100%;"></canvas> -->
        </div>
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

.general {
    display: flex;
    flex-direction: column;
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

.general h2 {
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
    display: flex;
    flex-direction: column;
    flex-wrap: wrap;
    gap: 0 2rem;
    padding-top: 0.5rem;
    padding-bottom: 0.5rem;
    border-bottom: 1px solid var(--border);
}

.price {
    display: flex;
    flex-direction: column;
    margin-top: 0.5rem;
    height: 40%;
    max-height: 20rem;
    overflow: hidden;
}

</style>