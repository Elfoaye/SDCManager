<script setup>
const props = defineProps(['setPage']);
</script>

<template>
    <nav>
        <section>
            <h2>Matériel</h2>
            <button @click="setPage('consult')">Consulter</button>
            <button @click="setPage('modif')">Modifier</button>
        </section>
        <section>
            <h2>Devis</h2>
            <button class="disabled">Consulter</button>
            <button class="disabled">Créer</button>
        </section>
        <section>
            <h2>Admin</h2>
            <button @click="setPage('admin')">Page Admin</button>
        </section>
        <button @click="setPage('params')" class="settings">Paramètres</button>
    </nav>
</template>

<style scoped>
nav{
    height: auto;
    width: clamp(5rem, 18vw, 12rem);
    flex-shrink: 0;
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
    list-style-type: none;
    margin: 0;
    padding: 1rem;
    background-color: var(--accent);
    border-right: 1px solid var(--border);
}

section {
    display: flex;
    flex-direction: column;
    align-items: stretch;
    width: 0 auto;
    padding-bottom: 0.5rem;
    background-color: var(--accent);
    border-bottom: 1px solid var(--border-accent);
}

h2 {
    font-size: 1.1rem;
    font-weight: 500;
    margin-top: 0;
    margin-bottom: 0.3rem;
    margin-left: 0.3rem;
}

button {
    text-align: start;
    padding: 0.5rem;
    font-size: 1rem;
    height: 2rem;
    width: 100%;
    border-radius: 0.5rem;
    background-color: var(--accent);
    color: var(--text);
    border: none;
    cursor: pointer;

    transition: all 0.1s;
}

button:hover {
    background-color: var(--accent-hover);

    transition: all 0.2s;
}

button.disabled{
    color: var(--text-muted);
    cursor: default;
}

button.disabled:hover{
    background-color: var(--accent)
}

.settings {
    margin-top: auto;
}

</style>