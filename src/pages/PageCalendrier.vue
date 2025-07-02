<script setup>
import FullCalendar from '@fullcalendar/vue3';
import dayGridPlugin from '@fullcalendar/daygrid';
import frLocale from '@fullcalendar/core/locales/fr';
import interactionPlugin from '@fullcalendar/interaction';

const calendarOptions = {
    plugins: [dayGridPlugin, interactionPlugin],
    initialView: 'dayGridMonth',
    locale: frLocale,
    events: [
        {
            title: 'Fête de la musique',
            start: '2025-07-02',
            end: '2025-07-06',
            id: 20250601
        },
        {
            title: 'Verticalété',
            start: '2025-06-28',
            end: '2025-07-04',
            id: 20250602
        },
        {
            title: 'Pôtes de marmots',
            start: '2025-07-08',
            end: '2025-07-09',
            id: 20250701
        }
    ],
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
}
</script>

<template>
    <div class="cal-container">
        <FullCalendar :options="calendarOptions" />
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