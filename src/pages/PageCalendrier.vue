<script setup>
function handleDayClick(date) {
    console.log('Afficher détails du jour :', date);
}

function handleEventClick(event) {
    console.log('Rediriger vers le devis :', event);
    
}
</script>

<template>
    
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