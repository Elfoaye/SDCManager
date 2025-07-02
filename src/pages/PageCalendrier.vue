<script setup>
import { reactive } from 'vue';
import { VueCal, addDatePrototypes } from 'vue-cal';
import 'vue-cal/style';

addDatePrototypes() 

const events = reactive([
  { 
    start: new Date(2025, 5, 21), 
    end: new Date(2025, 7, 21),
    title: 'Event 1' 
  },
  { 
    start: new Date().addHours(1), 
    end: new Date().addHours(2), 
    title: 'Event 2' 
  }
]);

function handleDayClick({ date }) {
    console.log('Afficher détails du jour :', date);
}

function handleEventClick(event) {
    console.log('Rediriger vers le devis :', event);
}
</script>

<template>
    <div class="cal-container">
        <VueCal
            :events="events"
            :time="false"
            default-view="month"
            locale="fr"
            @day-click="handleDayClick"
            @event-click="handleEventClick"
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

.vuecal {
    height: 98%;
    width: 98%;
    margin: 0.5rem;
    border: 1px solid var(--border);
    border-radius: 0.5rem;
    
}
</style>