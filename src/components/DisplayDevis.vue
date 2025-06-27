<script setup>
import { ref, computed } from 'vue';
import { useDevisStore } from '../composables/devisStore';
import SecretRib from './SecretRib.vue';

const store = useDevisStore();
const printRoot = ref(null);
defineExpose({ printRoot });

const ITEMS_PER_PAGE = 20;

const materielAssur = computed(() => {
    return store.selectedItems.reduce((sum, item) => sum + item.valeur, 0);
});

const materielCost = computed(() => {
    return store.selectedItems.reduce((sum, item) => sum + item.totalPrice, 0);
});

const finalCost = computed(() => {
    let price = store.utilitaries.techQty * store.utilitaries.techRate;
    price += store.utilitaries.transportKm * store.utilitaries.transportRate;
    if(store.utilitaries.membership && store.formulas) {
        price += store.formulas.membership;
    }

    price += materielCost.value;
    price += store.extraItems.reduce((sum, item) => sum + item.price, 0);

    price -= store.utilitaries.discountEuro;

    return price;
});

const paginatedItems = computed(() => {
  const pages = [];
  for (let i = 0; i < store.selectedItems.length; i += ITEMS_PER_PAGE) {
    pages.push(store.selectedItems.slice(i, i + ITEMS_PER_PAGE));
  }
  return pages;
});

const techTransport = computed(() => (store.utilitaries.techQty > 0)  || (store.utilitaries.transportKm > 0));

const yearMembership = computed(() => {
    const dateStr = store.devisInfos.date;

    if (!dateStr || typeof dateStr !== "string") return null;

    const [year, month, day] = dateStr.split("-").map(Number);

    if (
        Number.isNaN(day) || Number.isNaN(month) || Number.isNaN(year) ||
        day < 1 || day > 31 ||
        month < 1 || month > 12 ||
        year < 1000 || year > 9999
    ) {
        return null;
    }

    if(month > 8) {
        return `${year}/${year + 1}`
    }
    return `${year - 1}/${year}`
});
</script>

<template>
    <div ref="printRoot" class="all-pages">
        <div class="page">
            <header>
                <div class="infos">
                    <img src="../assets/LOGO_SDC.png">
                    <div class="general-info">
                        <p class="date">A Arvieux le {{ store.devisInfos.writeDate }}</p>
                        <div class="adress">
                            <p class="adress-title">Raison Sociale et Adresse de facturation :</p>
                            <div class="adress-content">
                                <p>{{ store.clientInfos.name }}</p>
                                <p style="white-space: pre-line;">{{ store.clientInfos.adress }}</p>
                            </div>
                        </div>  
                    </div>
                </div>
                <p class="context blue">Contribution mise à disposition de matériel son et éclairage</p>
                <p v-if="store.isFacture" class="nom-devis">Facture n°{{ store.devisInfos.id }}</p>
                <p v-else class="nom-devis">Devis {{ store.devisInfos.name }}</p>
            </header>
            <div class="body">
                <table>
                    <thead class="blue">
                        <tr>
                            <th v-if="techTransport">Dénomination</th>
                            <th v-else colspan="3">Dénomination</th>
                            <th v-if="techTransport">VLU</th>
                            <th v-if="techTransport">Unités</th>
                            <th>Total</th>
                        </tr>
                    </thead>
                    <tbody>
                        <tr v-if="store.utilitaries.techQty > 0">
                            <td>Technicien/jour</td>
                            <td>{{ store.utilitaries.techRate }}€</td>
                            <td>{{ store.utilitaries.techQty }}</td>
                            <td>{{ store.utilitaries.techRate * store.utilitaries.techQty }}€</td>
                        </tr>
                        <tr v-if="store.utilitaries.transportKm > 0">
                            <td>Transport</td>
                            <td>{{ store.utilitaries.transportRate }}€</td>
                            <td>{{ store.utilitaries.transportKm }}</td>
                            <td>{{ Number((store.utilitaries.transportRate * store.utilitaries.transportKm).toFixed(2)) }}€</td>
                        </tr>
                        <tr v-if="store.selectedItems.length > 0">
                            <td>Matériel (détails page suivante)</td>
                            <td colspan="2">Valeur à assurer : <span>{{ Number(materielAssur.toFixed(2)) }}€</span></td>
                            <td>{{ materielCost.toFixed(2) }}€</td>
                        </tr>
                        <tr v-if="store.utilitaries.membership">
                            <td colspan="3">Adhésion morale année scolaire {{ yearMembership }}</td>
                            <td>{{ store.formulas.membership }}€</td>
                        </tr>
                        <tr v-for="extra in store.extraItems">
                            <td colspan="3" style="white-space: pre-line;">{{ extra.name }}</td>
                            <td>{{ Number(extra.price.toFixed(2)) }}€</td>
                        </tr>
                        <tr v-if="store.utilitaries.discountEuro > 0" class="before-remise summ blue">
                            <td colspan="3">TOTAL avant remise</td>
                            <td>{{ (finalCost + store.utilitaries.discountEuro).toFixed(2) }}€</td>
                        </tr>
                        <tr v-if="store.utilitaries.discountEuro > 0" class="remise">
                            <td colspan="3">Remise</td>
                            <td>- {{ Number(store.utilitaries.discountEuro.toFixed(2)) }}€</td>
                        </tr>
                    </tbody>
                    <tfoot class="summ blue">
                        <tr>
                            <td colspan="3">TOTAL Général</td>
                            <td>{{ finalCost.toFixed(2) }}€</td>
                        </tr>
                    </tfoot>
                </table>
                <p>En vous remerciant pour votre confiance.</p>
            </div>
            <SecretRib v-if="store.isFacture" />

            <footer>
                <p class="legal">
                    Son des Cimes – 39 Imp. Du Pellas – 05350 Arvieux<br>
                    Association loi 1901 reconnue d’intérêt général n° W051000990 - Code APE : 9329Z<br>
                    Siret : 513 386 979 000 19 - &#9990; : 06 62 54 34 79 <br>
                    sondescimes@gmail.com - http://sondescimes.lescigales.org<br>
                    Non assujettie à la T.V.A.<br>
                </p>
                <div class="number">
                    <img src="../assets/QRCode.png">
                    <p>1</p>
                </div>
            </footer>
        </div>

        <div class="page" 
            v-for="(pageItems, index) in paginatedItems" 
            :key="index"
        >
            <header>
                <div class="infos">
                    <img src="../assets/LOGO_SDC.png">
                    <div class="general-info">
                        <p class="date">A Arvieux le {{ store.devisInfos.writeDate }}</p>
                        
                    </div>
                </div>
            </header>

            <table class="materiel">
                <thead class="blue">
                    <tr>
                        <th>Détail mise à disposition</th>
                        <th>Type</th>
                        <th>Val.Remp.</th>
                        <th>Valeur à Assurer</th>
                        <th>Contrib.</th>
                        <th>Unités</th>
                        <th>Durée</th>
                        <th>Total</th>
                    </tr>
                </thead>
                <tbody>
                    <tr v-for="item in pageItems" :key="item.id">
                        <td>{{ item.nom }}</td>
                        <td>{{ item.item_type }}</td>
                        <td>{{ Number(item.valeur) }}€</td>
                        <td>{{ Number((item.valeur * item.quantity).toFixed(2)) }}€</td>
                        <td>{{ Number(item.contrib.toFixed(2)) }}€</td>
                        <td>{{ item.quantity }}</td>
                        <td>{{ item.duration }}</td>
                        <td>{{ Number(item.totalPrice.toFixed(2)) }}€</td>
                    </tr>
                </tbody>
                <tfoot class="summ blue" v-if="index === paginatedItems.length - 1">
                    <tr>                   
                        <td colspan="3">TOTAL</td>
                        <td>{{ Number(materielAssur.toFixed(2)) }}€</td>
                        <td colspan="3"></td>
                        <td>{{ Number(materielCost.toFixed(2)) }}€</td>                        
                    </tr>
                </tfoot>
            </table>         
            <footer>
                <p class="legal">
                    Son des Cimes – 39 Imp. Du Pellas – 05350 Arvieux<br>
                    Association loi 1901 reconnue d’intérêt général n° W051000990 - Code APE : 9329Z<br>
                    Siret : 513 386 979 000 19 - &#9990; : 06 62 54 34 79 <br>
                    sondescimes@gmail.com - http://sondescimes.lescigales.org<br>
                    Non assujettie à la T.V.A.<br>
                </p>
                <div class="number">
                    <img src="../assets/QRCode.png">
                    <p>{{ index + 2 }}</p>
                </div>
            </footer>
        </div>
    </div>
</template>

<style scoped>
.all-pages {
    display: flex;
    justify-content: center;
    flex-wrap: wrap;
    max-width: 100%;
    gap: 1rem;
}

.page {
    width: 641px; /* 210mm - margin */ 
    height: 1028px; /* 297mm - margin */ 

    padding: 76px;
    padding-top: 57px;
    padding-bottom: 20px;

    color: rgb(17,145,148);
    background: white;

    overflow: hidden;
    position: relative;

    display: flex;
    flex-direction: column;
    font-size: 12px;
}

header {
    display: flex;
    flex-direction: column;
    margin-bottom: 1rem;
}

.infos {
    display: flex;
    justify-content: space-between;
    align-items: flex-start;
    margin-bottom: 1rem;
}

.infos img {
  height: 180px;
  object-fit: contain;
}

.general-info {
    display: flex;
    flex-direction: column;
    align-items: flex-end;
    text-align: right;
}

.date {
    font-size: 14px;
    margin: 5px 15px;
}

.adress {
    text-align: left;
    font-size: clamp(12px, 100%, 14px);
    font-weight: 700;
    padding: 12px;
    width: 270px;
    height: 120px;
    color: black;
    border: 1px solid black;
    border-radius: 20px;
}

.adress p {
    margin: 0;
}

.adress-title {
    font-size: 14px;
    font-weight: 400;
}

.adress-content {
    display: flex;
    flex-direction: column;
    justify-content: center;
    margin: 5px;
}

.context {
    text-align: right;
    font-weight: 500;
    padding-right: 12px;
    margin-bottom: 0;
}

.nom-devis {
    margin-top: 0;
}

table {
    box-sizing: border-box;
    border-collapse: collapse;
    width: 100%;
}

th, td {
  margin: 0;
  text-align: left;
}

tbody td {
    padding-left: 5px;
    padding-right: 5px;
    border: 1px solid rgb(17,145,148);
}

tbody tr.blue td {
    border: 0;
    border-left: 1px solid white;
}

tbody tr.blue td:first-child {
    border-left: 1px solid rgb(17,145,148);
}


th, tfoot.summ.blue td {
    padding: 5px;
    font-size: 16px;
}

.blue {
    color: white;
    background-color: rgb(17,145,148);
}

.blue th + th,
.blue td + td {
    border-left: 1px solid white;
}

.blue th:last-child,
.blue td:last-child {
    border-right: 1px solid rgb(17,145,148);
}

.before-remise {
    font-size: 14px;
}

.remise {
    color: red;
}

.materiel td {
    font-size: 11px;
}

.materiel th {
    font-size: 14px;
}

footer {
    display: flex;
    justify-content: end;
    align-items: center;
    gap: 1rem;
    margin-top: auto;
    height: fit-content;
}

footer p {
    text-align: center;
    color: gray;
}

footer img {
    object-fit: contain;
    height: 90px;
}

.number {
    display: flex;
    flex-direction: column;
    gap: 0;
}

.number p {
    color: black;
    margin: 0;
}
</style>