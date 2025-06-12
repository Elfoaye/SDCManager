<script setup>
import { useBreadcrumb } from '../composables/breadcrumb';
import ListeSelectionDevis from '../components/ListeSelectionDevis.vue';

const { setBreadcrumb } = useBreadcrumb();
setBreadcrumb([
    { label: 'Accueil', page: null },
    { label: 'Devis & Factures', page: 'devparcour' },
    { label: 'Modifier', page: 'devmodif' }
  ]);
</script>

<template>
    <div class="content">
        <div class="title">
            <h1>Editer le devis</h1>
        </div>
        <section class="base">
            <div class="tech">
                <label>Techniciens :
                    <input type="number" />
                </label>
                <label>Par Heure ?
                    <input type="checkbox" />
                </label>
                <label>Prix unitaire :
                    <input type="number" />
                </label>
                <label>Total :
                    <input type="number" />
                </label>
            </div>
            <div class="transport">
                <label>Transport (km):
                    <input type="number" />
                </label>
                <label>Prix unitaire :
                    <input type="number" />
                </label>
                <label>Total :
                    <input type="number" />
                </label>
            </div>
            <label>Adhésion ?
                <input type="checkbox" />
            </label>
        </section>
        <section class="materiel">
            <ListeSelectionDevis />
        </section>
        <section class="preview">

        </section>
        <section class="submit">
            <button>
                Sauvegarder
            </button>
            <button>
                Annuler
            </button>
            <button>
                Apperçu
            </button>
            <button>
                Télecharger
            </button>
        </section>
    </div>
</template>

<style scoped>
section {
    padding: 1rem;
    margin-bottom: 1rem;
    border-bottom: 1px solid var(--border);
}

section.base {
    display: flex;
    flex-direction: column;
    gap: 0.5rem
}

section.base div {
    display: grid;
    grid-template-columns: 1fr 1fr 1fr 1fr;
    gap: 1rem
}

section.base div label {
    display: flex;
    flex-direction: column;
}

section.materiel {
    max-height: 40vh;
}
</style>
