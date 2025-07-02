<script setup>
import { invoke } from '@tauri-apps/api/core';
import { computed, ref } from 'vue';
import FullCalendar from '@fullcalendar/vue3';
import dayGridPlugin from '@fullcalendar/daygrid';
import frLocale from '@fullcalendar/core/locales/fr';
import interactionPlugin from '@fullcalendar/interaction';

const calendarRef = ref(null);
const listFactures = ref([]);

const calendarEvents = computed(() =>
    listFactures.value.map((facture) => ({
        id: facture.id,
        title: facture.nom,
        start: facture.date,
        end: addDays(facture.date, facture.durée),
        allDay: true
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
    }
};

function handleEventClick(event) {
    console.log('Rediriger vers le devis :', event.id);
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