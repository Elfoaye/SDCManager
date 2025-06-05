<script setup>
import { ref, onMounted } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';
import { useBreadcrumb } from '../composables/breadcrumb';
const { breadcrumb } = useBreadcrumb();

const props = defineProps(['current_page','last_page','setPage']);

const isAdmin = ref(false);

function logOutAdmin() {
    invoke('log_out_admin');
}

listen('log_in_admin', (event) => {
    isAdmin.value = event.payload;
});

onMounted(() => {
    invoke('isAdmin').then((value) => isAdmin.value = value);
});
</script>

<template>
    <header>
        <div class="topnav" :class="{ admin: isAdmin }">
            <img src="../assets/LOGO_SDC.png">

            <button v-if="isAdmin" @click="logOutAdmin">Mode Admin</button>
        </div>
        <nav class="path">
            <button @click="setPage(last_page)">&#8617;</button>
            <div
                v-for="(step, index) in breadcrumb"
                :key="index"
                class="step"
            >
                <button @click="setPage(step.page)">{{ step.label }}</button> 
                <span v-if="index < breadcrumb.length - 1">></span>
            </div>
        </nav>
    </header>
</template>

<style scoped>
header {
    display: flex;
    align-items: center;
    border-bottom: 1px solid var(--border);
    height: clamp(3rem, 8vh, 5rem);
}

.topnav {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    width: clamp(5rem, 18vw, 12rem);
    height: 100%;
    padding: 1rem;
}

.topnav img {
    max-height: 80%;
}

.admin {
    height: 100%;
    display: flex;
    align-items: center;
    background: linear-gradient(-90deg, var(--background), var(--warning));
}

button {
    border: 0;
    hyphens: auto;
    white-space: normal;
    font-size: clamp(0.6rem, 1.5vw, 1rem);
    padding: 1rem;
    background-color: var(--background);
    color: var(--text);
    cursor: pointer;
    border-radius: 0.5rem;
    max-height: 100%;

    transition: all 0.15s;
}

button:hover {
    background-color: var(--accent);

    transition: all 0.15s;
}

.admin button {
    display: flex;
    align-items: center;
    max-height: 60%;
    padding: 0.5rem;
    border: 2px solid var(--warning);
    border-radius: 2rem;
    margin-right: 1rem;
    font-weight: 600;
}

.admin button:hover {
    background-color: var(--disabled);
    border-color: var(--background-error);
}

.path {
    display: flex;
    padding-left: 1rem;
    max-height: 100%;
}


</style>