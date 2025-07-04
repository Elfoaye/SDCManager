<script setup>
import { ref, onMounted } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';
import { useBreadcrumb } from '../composables/breadcrumb';
const { breadcrumb } = useBreadcrumb();

const props = defineProps(['setPage']);
const emit = defineEmits(['cancel']);

const isAdmin = ref(false);
const confirm = ref(false);

async function logOutAdmin() {
    await invoke('log_out_admin');
    confirm.value = false;
}

listen('log_in_admin', (event) => {
    isAdmin.value = event.payload;
});

onMounted(() => {
    invoke('is_admin').then((value) => isAdmin.value = value);
});
</script>

<template>
    <div v-if="confirm" class="confirm">
        <div class="pop-up">
            <p>Êtes-vous sûr de vouloir quitter le <span>Mode Admin</span> ?</p>

            <div class="confirm-buttons">
                <button @click="logOutAdmin" class="submit">Confirmer</button>
                <button @click="confirm=false" class="cancel">Annuler</button>
            </div>
        </div>
    </div>
    <header>
        <div class="topnav" :class="{ admin: isAdmin }">
            <img src="../assets/Logo.png">

            <button v-if="isAdmin" @click="confirm=true">Mode Admin</button>
        </div>
        <nav class="path">
            <button @click="emit('cancel')">&#8617;</button>
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
    width: clamp(9rem, 18vw, 14rem);
    padding-left: 1rem;
    height: 100%;
}

.topnav img {
    max-height: 100%;
}

.topnav.admin {
    box-sizing: border-box;
    background: linear-gradient(to left, var(--background), var(--warning));
}

.path {
    max-height: 100%;
}

button {
    border: 0;
    hyphens: auto;
    white-space: normal;
    font-size: clamp(0.6rem, 2vw, 1rem);
    background-color: var(--background);
    max-height: 90%;
    width: fit-content;

    transition: all 0.15s;
}

button:hover {
    background-color: var(--accent);

    transition: all 0.15s;
}

.admin button {
    display: flex;
    align-items: center;
    max-height: 80%;
    width: auto;
    padding: 0.5rem;
    border: 2px solid var(--warning);
    border-radius: 2rem;
    font-weight: 600;
}

.admin button:hover {
    background-color: var(--disabled);
    border-color: var(--background-error);
}

.path {
    display: flex;
    max-height: 100%;
}

.confirm {
    position: fixed;
    top: 0;
    left: 0;
    right: 0;
    bottom: 0;
    display: flex;
    justify-content: center;
    align-items: center;
    background-color: rgba(0, 0, 0, 0.5);
    z-index: 1000;
}

.pop-up {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    max-height: 5rem;
    background-color: var(--background-alt);
    border: 1px solid var(--border-accent);
    border-radius: 0.5rem;
    padding: 1em;
    padding-bottom: 2rem;
}

.pop-up button {
    height: 100%;
    padding: 1rem;
    margin-right: 1rem;
    width: 8rem;
    border-radius: 0.5rem;

    transition: all 0.2s;
}

.submit {
    background-color: var(--accent-hover);
}

.cancel {
    background-color: var(--disabled);
}

span {
    font-weight: 600;
}
</style>