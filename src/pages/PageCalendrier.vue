<script setup>
import { invoke } from '@tauri-apps/api/core';
import { computed, ref, watch } from 'vue';
import { useBreadcrumb } from '../composables/breadcrumb';
import FullCalendar from '@fullcalendar/vue3';
import dayGridPlugin from '@fullcalendar/daygrid';
import frLocale from '@fullcalendar/core/locales/fr';
import interactionPlugin from '@fullcalendar/interaction';

const { setDocument } = defineProps(['setDocument']);

const { setBreadcrumb } = useBreadcrumb();
setBreadcrumb([
    { label: 'Accueil', page: null },
    { label: 'Devis & Devis', page: 'devparcour' },
    { label: 'Calendrier', page: 'cal' }
]);

const calendarRef = ref(null);
const listDevis = ref([]);
const dailyDevis = ref({});

const selectedDay = ref(null);

function stringToColor(str) {
    let hash = 0;
    for (let i = 0; i < str.length; i++) {
        hash = str.charCodeAt(i) + ((hash << 5) - hash);
    }
    const hue = Math.abs(hash) % 360;
    return `hsl(${hue}, 60%, 65%)`;
}

const calendarEvents = computed(() =>
    listDevis.value.map((devis) => ({
        id: devis.id,
        title: devis.nom,
        start: devis.date,
        end: addDays(devis.date, devis.durée),
        allDay: true,
        backgroundColor: stringToColor(devis.client_nom)
    }))
);

invoke('get_devis_summaries').then((devis) => { 
    listDevis.value = devis.filter(d => d.etat && d.etat.toLowerCase().includes('valide'));

    const calendarApi = calendarRef.value?.getApi();

    if (calendarApi) {
        calendarApi.removeAllEvents();
        calendarApi.addEventSource(calendarEvents.value);
    }
});

function addDays(dateStr, days) {
    const date = new Date(dateStr);
    date.setDate(date.getDate() + days);
    return date.toISOString().split('T')[0];
}

const calendarOptions = {
    plugins: [dayGridPlugin, interactionPlugin],
    initialView: 'dayGridMonth',
    locale: frLocale,
    eventClick(info) {
        handleEventClick(info.event);
    },
    dateClick(info) {
        handleDayClick(info.dateStr);
    },
    eventDidMount(info) {
        const el = info.el.querySelector('.fc-event-title');
        if (el) {
            el.style.background = 'rgba(0, 0, 0, 0.4)';
            el.style.color = 'white';
            el.style.padding = '0 5px';
            el.style.borderRadius = '2px';
        }
  }
};

function handleEventClick(event) {
    setDocument({ id: event.id, facture: false });
}

async function handleDayClick(date) {
    if(!date || selectedDay.value === date) {
        selectedDay.value = null;
        return;
    }

    selectedDay.value = date;
    dailyDevis.value = {};

    const dayDev = listDevis.value.filter(devis => {
        const start = new Date(devis.date);
        const end = new Date(start);
        end.setDate(start.getDate() + devis.durée);
        const d = new Date(date);
        return d >= start && d < end;
    });

    for (const devis of dayDev) {
        try {
            const materiel = await invoke('load_devis_materiel', { devisId: devis.id });
            dailyDevis.value[devis.id] = {
                id: devis.id,
                nom: devis.nom,
                client_nom: devis.client_nom,
                materiel
            };
        } catch (err) {
            console.error(`Erreur chargement matériel devis ${devis.id}`, err);
        }
    }
}

function formatDate(dateStr) {
    const date = new Date(dateStr);
    return new Intl.DateTimeFormat('fr-FR', {
        day: 'numeric',
        month: 'long',
        year: 'numeric'
    }).format(date);
}

watch(selectedDay, (newDate, oldDate) => {
    if (oldDate) {
        const oldEl = document.querySelector(`.fc-daygrid-day[data-date="${oldDate}"]`);
        if (oldEl) oldEl.classList.remove('selected');
    }

    if (newDate) {
        const newEl = document.querySelector(`.fc-daygrid-day[data-date="${newDate}"]`);
        if (newEl) newEl.classList.add('selected');
    }
});
</script>

<template>
    <div class="cal-container">
        <FullCalendar 
            class="calendar"
            ref="calendarRef"
            :options="calendarOptions" 
        />
        <Transition name="expand-height" mode="out-in">
            <div class="itemCard" v-if="selectedDay">
                <div class="title">
                    <h1>{{ formatDate(selectedDay) }}</h1>
                    <button class="back" @click="handleDayClick(null)">X</button>
                </div>
                <div class="data">
                    <div v-for="devis in dailyDevis" class="devis" @click="setDocument({ id: devis.id, facture: false })">
                        <div class="name">
                            <h2>{{ devis.id + ' ' + devis.nom }}</h2>
                            <h3>{{ devis.client_nom }}</h3>
                        </div>
                        <p v-for="item in devis.materiel"><span>{{ item.quantité }}</span> {{ item.nom }}</p>
                    </div>
                </div>
        </div>
        </Transition>
    </div>
    
</template>

<style scoped>
.cal-container {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    height: 100%;
    width: 100%;
    overflow: hidden;
}

.calendar {
    flex: 1; 
    width: 100%;
    padding: 1rem;
    overflow-y: auto;
}

::v-deep(.fc-event) {
    transition: all 0.2s ease;
}

::v-deep(.fc-event:hover) {
  filter: brightness(0.9);
  transform: scale(1.02);
  transition: all 0.2s ease;
  cursor: pointer;
}

::v-deep(.fc-daygrid-day:hover:not(:has(.fc-event:hover))) {
  background-color: var(--surface-hover);
  transition: background-color 0.2s ease;
  cursor: pointer;
}

::v-deep(.fc-daygrid-day.selected) {
    background-color: var(--surface-selected);
    outline: 2px solid var(--border-accent);
    outline-offset: -2px;
    border-radius: 4px;
}

.itemCard {
    flex: 1;
    width: 100%;
    max-width: 100%;
    border: 0;
    padding: 0;
    overflow: hidden;
}

.title {
    display: flex;
    justify-content: space-between;
    align-items: center;
}

.back {
    height: 3rem;
    width: 3rem;
}

.data {
    overflow-y: auto;
    display: flex;
    justify-content: space-around;
    flex-wrap: wrap;
    gap: 1rem;
}

.devis {
    flex: 1;
    padding: 1rem;
    border: 1px solid var(--border);
    border-radius: 0.5rem;
}

.devis:hover {
    cursor: pointer;
    background-color: var(--surface-hover);
}

.devis p {
    margin-left: 1rem;
}

.expand-height-enter-active,
.expand-height-leave-active {
  transition: transform 0.3s ease, opacity 0.3s ease;
  will-change: transform, opacity;
}
.expand-height-enter-from,
.expand-height-leave-to {
  opacity: 0;
  transform: scaleY(0);
  transform-origin: bottom;
}
.expand-height-enter-to,
.expand-height-leave-from {
  opacity: 1;
  transform: scaleY(1);
  transform-origin: bottom;
}
</style>