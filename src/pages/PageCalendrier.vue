<script setup>
import { invoke } from '@tauri-apps/api/core';
import { computed, ref } from 'vue';
import { useBreadcrumb } from '../composables/breadcrumb';
import FullCalendar from '@fullcalendar/vue3';
import dayGridPlugin from '@fullcalendar/daygrid';
import frLocale from '@fullcalendar/core/locales/fr';
import interactionPlugin from '@fullcalendar/interaction';

const { setDocument } = defineProps(['setDocument']);

const { setBreadcrumb } = useBreadcrumb();
setBreadcrumb([
    { label: 'Accueil', page: null },
    { label: 'Devis & Factures', page: 'devparcour' },
    { label: 'Planning', page: 'cal' }
]);


const calendarRef = ref(null);
const listFactures = ref([]);

function stringToColor(str) {
    let hash = 0;
    for (let i = 0; i < str.length; i++) {
        hash = str.charCodeAt(i) + ((hash << 5) - hash);
    }
    const hue = Math.abs(hash) % 360;
    return `hsl(${hue}, 60%, 65%)`;
}

const calendarEvents = computed(() =>
    listFactures.value.map((facture) => ({
        id: facture.id,
        title: facture.nom,
        start: facture.date,
        end: addDays(facture.date, facture.durée),
        allDay: true,
        backgroundColor: stringToColor(facture.client_nom)
    }))
);

invoke('get_factures_summaries').then((factures) => { 
    listFactures.value = factures; 

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
    setDocument({ id: event.id, facture: true });
}

function handleDayClick(date) {
    console.log('Afficher détails du jour :', date);
    console.table(calendarEvents.value);
}
</script>

<template>
    <div class="cal-container">
        <FullCalendar 
            ref="calendarRef"
            :options="calendarOptions" 
        />
    </div>
</template>

<style>
.cal-container {
    display: flex;
    align-items: center;
    justify-content: center;
    height: 100%;
    width: 100%;
    overflow: hidden;
}
</style>