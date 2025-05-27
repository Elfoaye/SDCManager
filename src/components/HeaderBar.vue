<script setup>
import { useBreadcrumb } from '../composables/breadcrumb';
const { breadcrumb } = useBreadcrumb();

const props = defineProps(['current_page','last_page','setPage']);
</script>

<template>
    <header>
        <div class="topnav">
            <img src="../assets/LOGO_SDC.png">
        </div>
        <nav class="path">
            <button class="back" @click="setPage(last_page)">&#8617;</button>
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
    width: clamp(5rem, 18vw, 12rem);
    height: 100%;
    padding: 1rem;
}

.topnav img {
    max-height: 80%;
}

.path {
    display: flex;
    gap: 0.3rem;
    padding-left: 1rem;
    max-height: 100%;
}

button {
    border: 0;
    font-size: 1rem;
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
</style>