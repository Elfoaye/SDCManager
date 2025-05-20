<script setup>
import { ref } from 'vue';

const list_content = ref(
    [{nom:"Stairville Retro Flat Par 18x10W RGBWA UV",  type:"Lumière", disponible:4, total:12, contribution:9, valeur:269},
    {nom:"JBL EON710",  type:"Diff", disponible:2, total:2, contribution:15, valeur:535}]
)
</script>

<template>
    <div class="content">
        <h1>Voici tout le matériel</h1>

        <ul>
            <li class = "head">
                <p>Nom</p>
                <p>Catégorie</p>
                <p>Disponible</p>
                <p>Total</p>
                <p>Prix/Journée</p>
                <p>Valeur</p>
            </li>
            <li v-for="item in list_content">
                <p>{{ item.nom }}</p>
                <p>{{ item.type }}</p>
                <p>{{ item.disponible }}</p>
                <p>{{ item.total }}</p>
                <p>{{ item.contribution }}</p>
                <p>{{ item.valeur }}</p>
            </li>
        </ul>
    </div>
</template>

<style scoped>
.content {
    flex-grow: 1;
    display: flex;
    flex-direction: column;
    max-width: 60rem;
    padding: 2rem;
}

ul {
    display: flex;
    flex-direction: column;
    list-style-type: none;
    margin: 0;
    padding: 0;
}

li {
    display: grid;
    grid-template-columns: 3fr 1fr 1fr 1fr 1fr 1fr;
    padding: 0 0.5rem;
    gap: 1rem;
    align-items: center;
    border-bottom: 1px solid var(--border);
}

li:not(.head):nth-child(even) {
  background-color: var(--background-alt);
}

li p {
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
}

li.head {
    border-bottom: 1px solid var(--border-accent);
    font-weight: 500;
}
</style>