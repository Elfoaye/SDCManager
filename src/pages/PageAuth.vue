<script setup>
import { invoke } from '@tauri-apps/api/core';
import { ref } from 'vue';
import { useBreadcrumb } from '../composables/breadcrumb';

const { setBreadcrumb } = useBreadcrumb();
setBreadcrumb([
    { label: 'Accueil', page: null },
    { label: 'Authentication', page: 'auth' }
  ]);

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
        <h1>Cette section requiert des droits Admin</h1>
        <h2>Identifiez-vous pour continuer</h2>
        <div class="submit">
            <div class="field">
                <input class="searchbar" v-model="password" type="password"  @input="password_error=''" placeholder="Mot de passe..."/>
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

.submit {
    display: grid;
    grid-template-columns: 1fr 1fr;
    gap: 1rem;
    width: 100%;
    max-width: 30rem;
    margin-top: 2rem;
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
}

button {
    justify-self: end;
    width: 100%;
    height: 100%;
    max-width: 8rem;
}

button.cancel {
    justify-self: start;
    background-color: var(--disabled);
}

.search {
    width: 100%;
    display: flex;
    align-items: center;
    justify-content: center;
}

.searchbar {
    width: 100%;
    padding: 0.5rem;
}

.error {
    color: var(--error);
    margin: 0;
    margin-top: 0.5rem;
}
</style>