<script setup>
import { useBreadcrumb } from '../composables/breadcrumb';

const { setBreadcrumb } = useBreadcrumb();
setBreadcrumb([
    { label: 'Accueil', page: null },
]);

const { setDocument, setPage } = defineProps(['setDocument', 'setPage']);
</script>

<template>
    <div class="all">
        <div class="title">
            <h1>Bonjour Son Des Cimes !</h1>
        </div>
        <div class="content-body">
            <section class="devis">
                <button @click="setDocument({id: 0, facture: false}, true)">
                    <svg width="240" height="240" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
                        <path d="M19 13V15C19 17.8284 19 19.2426 18.1213 20.1213C17.2426 21 15.8284 21 13 21H11C8.17157 21 6.75736 21 5.87868 20.1213C5 19.2426 5 17.8284 5 15V9C5 6.17157 5 4.75736 5.87868 3.87868C6.75736 3 8.17157 3 11 3H12" stroke="currentColor" stroke-linecap="round"/>
                        <path d="M18 3L18 9" stroke="currentColor" stroke-linecap="round"/>
                        <path d="M21 6L15 6" stroke="currentColor" stroke-linecap="round"/>
                        <path d="M9 13L15 13" stroke="currentColor" stroke-linecap="round"/>
                        <path d="M9 9L13 9" stroke="currentColor" stroke-linecap="round"/>
                        <path d="M9 17L13 17" stroke="currentColor" stroke-linecap="round"/>
                    </svg>
                    Nouveau devis
                </button>
                <div class="list">
                    <h2>Derniers devis : </h2>
                    <ul>
                        <li>Fête de la musique</li>
                        <li>Fête de la musique</li>
                        <li>Fête de la musique</li>
                    </ul>
                </div>
            </section>

            <section class="facture">
                <button @click="setPage('cal')">
                    <svg width="240" height="240" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
                        <path d="M17 3L17 7" stroke="currentColor" stroke-width="1.2" stroke-linecap="round"/>
                        <path d="M7 3L7 7" stroke="currentColor" stroke-width="1.2" stroke-linecap="round"/>
                        <path d="M3 10C3 8.11438 3 7.17157 3.58579 6.58579C4.17157 6 5.11438 6 7 6H17C18.8856 6 19.8284 6 20.4142 6.58579C21 7.17157 21 8.11438 21 10V11H3V10Z" stroke="currentColor" stroke-width="1.2"/>
                        <rect x="3" y="6" width="18" height="15" rx="2" stroke="currentColor" stroke-width="1.2"/>
                        <path d="M6 15H10" stroke="currentColor" stroke-opacity="0.25" stroke-width="1.2" stroke-linecap="round"/>
                        <path d="M14 15H18" stroke="currentColor" stroke-opacity="0.25" stroke-width="1.2" stroke-linecap="round"/>
                        <path d="M6 18H10" stroke="currentColor" stroke-opacity="0.25" stroke-width="1.2" stroke-linecap="round"/>
                        <path d="M14 18H18" stroke="currentColor" stroke-opacity="0.25" stroke-width="1.2" stroke-linecap="round"/>
                    </svg>
                    Calendrier
                </button>
                <div class="list">
                    <h2>Prochains évenements : </h2>
                    <ul>
                        <li>Fête de la musique</li>
                        <li>Fête de la musique</li>
                        <li>Fête de la musique</li>
                    </ul>
                </div>
            </section>
            <section class="materiel">
                <button @click="setPage('consult')">
                    <svg width="240" height="240" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
                        <circle cx="11" cy="11" r="6" stroke="currentColor"/>
                        <path d="M11 8C10.606 8 10.2159 8.0776 9.85195 8.22836C9.48797 8.37913 9.15726 8.6001 8.87868 8.87868C8.6001 9.15726 8.37913 9.48797 8.22836 9.85195C8.0776 10.2159 8 10.606 8 11" stroke="currentColor" stroke-linecap="round"/>
                        <path d="M20 20L17 17" stroke="currentColor" stroke-linecap="round"/>
                    </svg>
                    Consulter
                </button>
                <div class="list">
                    <h2>Dernières sorties : </h2>
                    <ul>
                        <li>Ampli Marshall 70th DSL40CR BLK+ Stompware</li>
                        <li>Ampli Marshall 70th DSL40CR BLK+ Stompware</li>
                        <li>Ampli Marshall 70th DSL40CR BLK+ Stompware</li>
                    </ul>
                </div>
            </section>
        </div>
    </div>
</template>

<style scoped>
.all {
    display: flex;
    flex-direction: column;
    justify-content: center;
    width: 100%;
    height: 100%;
}

.content-body {
    width: 100%;
    height: 100%;
    display: flex;
    justify-content: center;
    align-items: center;
    overflow: hidden;
}

section {
    flex: 1;
    height: 100%;
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    gap: 2rem;
    overflow: hidden;
}

section button {
    height: 8rem;
    width: 8rem;
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 0.5rem;
}

.list {
    max-height: 50%;
}

ul {
    max-height: 100%;
    overflow-y: auto;
}
</style>