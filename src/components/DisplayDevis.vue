<script setup>
import { ref, computed } from 'vue';
import { useDevisStore } from '../composables/devisStore';

const props = defineProps(['devis']);
const store = useDevisStore();
const printRoot = ref(null);
defineExpose({ printRoot });

const ITEMS_PER_PAGE = 20;

const IBAN = 'FR76' + '\u200B' + '1680' + '\u200B' + '7001' + '\u200B' + '3131' + '\u200B' + '7631' + '\u200B' + '1521' + '\u200B' + '577';

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
</script>

<template>
    <div ref="printRoot" class="all">
        <div class="page">
            <header>
                <div class="infos">
                    <img src="../assets/LOGO_SDC.png">
                    <div class="general-info">
                        <p class="date">A Arvieux le {{ store.devisInfos.writeDate }}</p>
                        <div class="adress">
                            <p>{{ store.clientInfos.name }}</p>
                            <p style="white-space: pre-line;">{{ store.clientInfos.adress }}</p>
                        </div>  
                    </div>
                </div>
                <p class="context blue">Contribution mise à disposition de matériel Son et éclairage</p>
                <p class="nom-devis">Facture n°{{ store.devisInfos.id }}</p>
            </header>
            <div class="body">
                <table>
                    <thead class="blue">
                        <tr>
                            <th>Dénomination</th>
                            <th>VLU</th>
                            <th>Unités</th>
                            <th>Total</th>
                        </tr>
                    </thead>
                    <tbody>
                        <tr>
                            <td>Technicien/jour</td>
                            <td>{{ store.utilitaries.techRate }}€</td>
                            <td>{{ store.utilitaries.techQty }}</td>
                            <td>{{ store.utilitaries.techRate * store.utilitaries.techQty }}€</td>
                        </tr>
                        <tr>
                            <td>Transport</td>
                            <td>{{ store.utilitaries.transportRate }}€</td>
                            <td>{{ store.utilitaries.transportKm }}</td>
                            <td>{{ store.utilitaries.transportRate * store.utilitaries.transportKm }}€</td>
                        </tr>
                        <tr v-if="store.selectedItems.length > 10">
                            <td colspan="3">Matériel (détails page suivante)</td>
                            <td>{{ materielCost.toFixed(2) }}€</td>
                        </tr>
                        <tr v-else-if="store.selectedItems.length > 0" class="Materiel">
                            <td colspan="4">
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
                                        <tr v-for="item in store.selectedItems">
                                            <td>{{ item.nom }}</td>
                                            <td>{{ item.item_type }}</td>
                                            <td>{{ Number(item.valeur.toFixed(2)) }}€</td>
                                            <td>{{ Number((item.valeur * item.quantity).toFixed(2)) }}€</td>
                                            <td>{{ Number(item.contrib.toFixed(2)) }}€</td>
                                            <td>{{ item.quantity }}</td>
                                            <td>{{ item.duration }}</td>
                                            <td>{{ Number(item.totalPrice.toFixed(2)) }}€</td>
                                        </tr>
                                    </tbody>
                                    <tfoot class="blue">
                                        <tr>                   
                                            <td>TOTAL</td>
                                            <td></td>
                                            <td></td>
                                            <td>{{ Number(materielAssur.toFixed(2)) }}€</td>
                                            <td></td>
                                            <td></td>
                                            <td></td>
                                            <td>{{ Number(materielCost.toFixed(2)) }}€</td>                        
                                        </tr>
                                    </tfoot>
                                </table>
                            </td>
                        </tr>
                        <tr v-if="store.utilitaries.membership">
                            <td colspan="3">Adhésion morale</td>
                            <td>25€</td>
                        </tr>
                        <tr v-for="extra in store.extraItems">
                            <td colspan="3">{{ extra.name }}</td>
                            <td>{{ Number(extra.price.toFixed(2)) }}€</td>
                        </tr>
                        <tr v-if="store.utilitaries.discountEuro > 0">
                            <td colspan="3">Geste commercial</td>
                            <td>{{ Number(store.utilitaries.discountEuro.toFixed(2)) }}€</td>
                        </tr>
                    </tbody>
                    <tfoot class="summ blue">
                        <tr>
                            <td>TOTAL Général</td>
                            <td></td>
                            <td></td>
                            <td>{{ finalCost.toFixed(2) }}€</td>
                        </tr>
                    </tfoot>
                </table>
                <p>En vous remerciant pour votre confiance.</p>
            </div>

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
            v-if="store.selectedItems.length > 10"
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
                        <td>TOTAL</td>
                        <td></td>
                        <td></td>
                        <td>{{ Number(materielAssur.toFixed(2)) }}€</td>
                        <td></td>
                        <td></td>
                        <td></td>
                        <td>{{ Number(materielCost.toFixed(2)) }}€</td>                        
                    </tr>
                </tfoot>
            </table>

            <div v-if="index === paginatedItems.length - 1" class="rib">
                <h3>Relevé d'identité bancaire / Bank details statement</h3>
                <p>IBAN : <span>{{ IBAN }}</span></p>
                <p>BIC : <span>CCBPFRPPGRE</span></p>
                <p>Code Banque : <span>16807</span></p>
                <p>Code Guichet : <span>00131</span></p>
                <p>N° du compte : <span>31763115215</span></p>
                <p>Clé RIB : <span>77</span></p>
                <p>Domiciliation : <span>BPAURA GUILLESTRE</span></p>
            </div>
            
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
.all {
    display: flex;
    flex-direction: column;
    gap: 1rem;
}

.page {
    width: 641px; /* 210mm - margin */ 
    height: 1028px; /* 297mm - margin */ 

    padding: 76px;
    padding-top: 57px;
    padding-bottom: 20px;

    color: black;
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
    color: gray;
    font-weight: 500;
}

.adress {
    text-align: left;
    font-size: 14px;
    padding: 12px;
    width: 230px;
    height: 100px;
    border: 1px solid black;
    border-radius: 20px;
}

.adress p {
    margin: 0;
}

.blue {
    color: white;
    background-color: deepskyblue;
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
    border-collapse: collapse;
    width: 100%;
}

th, td {
  margin: 0;
  text-align: left;
  border: 1px solid deepskyblue;
}

td {
    padding-left: 5px;
    padding-right: 5px;
}

th, tfoot.summ.blue {
    padding: 0;
    padding-left: 5px;
    font-size: 16px;
}

.materiel td {
    font-size: 11px;
}

.materiel th {
    font-size: 14px;
}

.rib {
    display: flex;
    flex-wrap: wrap;
    margin-top: 30px;
    gap: 0 20px;
    border: solid 1px dimgray;
}

.rib h3 {
    width: 100%;
    font-size: 14px;
    padding-left: 5px;
    background-color: dimgray;
    color: white;
}

.rib p {
    padding-left: 5px;
    color: dimgray;
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