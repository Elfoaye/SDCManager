<script setup>
import { invoke } from '@tauri-apps/api/core';
import { ref } from 'vue';

const props = defineProps(['redirect','setPage']);
const emit = defineEmits(['cancel'])

const password = ref('');
const password_error = ref('');

async function confirmPassword() {
    if(password.value === '') {
        password_error.value = "Veuillez entrer le mot de passe pour continuer";
        return;
    }

    try {
        await invoke('log_in_admin', { password: password.value});
        if(invoke('is_admin')) {
            password_error.value = null;
            props.setPage(props.redirect);
        } else {
            password_error.value = "Authentification échouée, veuillez réessayer"
        }
    } catch(err) {
        password_error.value = err;
        password.value='';
    }
}
</script>

<template>
    <div class="content">
        <h1>Cette section requière des droits admin</h1>
        <h2>Identifiez-vous pour continuer</h2>
        <div class="submit">
            <div class="field">
                <input v-model="password" type="text"  @input="password_error=''" placeholder="Mot de passe..."/>
                <p v-if="password_error" class="error">{{ password_error }}</p>
            </div>
            <button class="confirm" @click="confirmPassword">Confirmer</button>
            <button class="cancel" @click="emit('cancel')">Annuler</button>
        </div>
    </div>
</template>

<style scoped>
.content {
    flex-grow: 1;
    display: flex;
    flex-direction: column;
    justify-content: center;
    align-items: center;
}

h1 {
    font: inherit;
    font-size: 1.5rem;
    font-weight: 500;
    margin: 0;
}

h2 {
    font: inherit;
    font-size: 1.2rem;
    margin-top: 0;
}

.submit {
    display: grid;
    grid-template-columns: 1fr 1fr;
    gap: 1rem;
    width: 100%;
    max-width: 30rem;
}

.submit .field {
    grid-column: span 2;
    display: flex;
    flex-direction: column;
    align-items: center;
    width: 100%;
}

input {
    align-self: center;
    width: 100%;
    max-width: 20rem;
    background-color: var(--background-alt);
    color: var(--text);
    border: 1px solid var(--border);
    border-radius: 0.3rem;
    padding: 0.5rem;
}

button {
    justify-self: end;
    width: 100%;
    height: 100%;
    padding: 1rem;
    margin-right: 1rem;
    max-width: 8rem;
    color: var(--text);
    background-color: var(--accent);
    border: 1px solid var(--border);
    border-radius: 0.5rem;

    transition: all 0.2s;
}

button:hover {
    background-color: var(--surface-selected);
    cursor: pointer;
    
    transition: all 0.2s;
}

button.cancel {
    justify-self: start;
    background-color: var(--disabled);
}

.error {
    color: var(--error);
    margin: 0;
    margin-top: 0.5rem;
}
</style>