<script setup>
import { useBreadcrumb } from '../composables/breadcrumb';

const { setBreadcrumb } = useBreadcrumb();
setBreadcrumb([
    { label: 'Accueil', page: null },
    { label: 'Admin', page: 'admin' }
  ]);
</script>

<template>
    <div class="content">
        <h1>Page Admin</h1>
        <div class="params">
            <section>
                <h2>Materiel</h2>
                <div class="formulas">
                    <h3>Formules</h3>
                    <label>Contribution de base (multiple la valeur) :
                        <input type="number" />
                    </label>
                    <p>(X % du la valeur)</p>
                    <label>Contribution suivante (multiple la contribution de base) :
                        <input type="number" />
                    </label>
                    <p>(X % de la contribution de base)</p>
                </div>
                <div class="Types">
                    <button>Ajouter</button>
                    <ul>
                        <li>Accessoires</li>
                    </ul>
                </div>
            </section>
            <section>
                <h2>Admin</h2>
                <div class="password">
                    <button>Changer le mot de passe</button>
                </div>
            </section>
        </div>
    </div>
</template>

<style scoped>
.content {
    display: flex;
    flex-direction: column;
}
</style>