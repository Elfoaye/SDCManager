<script setup>
import { ref } from 'vue';

const props = defineProps(['devis']);

const materiel = ref([
    {nom: 'Tente 3x3', item_type:'Acessoires', valeur:2500.00, contrib: 65.00, quantité: 2, totalPrice: 130 },
    {nom: 'Tente 3x3', item_type:'Acessoires', valeur:2500.00, contrib: 65.00, quantité: 2, totalPrice: 130 },
    {nom: 'Tente 3x3', item_type:'Acessoires', valeur:2500.00, contrib: 65.00, quantité: 2, totalPrice: 130 },
    {nom: 'Tente 3x3', item_type:'Acessoires', valeur:2500.00, contrib: 65.00, quantité: 2, totalPrice: 130 },
    {nom: 'Tente 3x3', item_type:'Acessoires', valeur:2500.00, contrib: 65.00, quantité: 2, totalPrice: 130 },
    {nom: 'Tente 3x3', item_type:'Acessoires', valeur:2500.00, contrib: 65.00, quantité: 2, totalPrice: 130 },
    {nom: 'Tente 3x3', item_type:'Acessoires', valeur:2500.00, contrib: 65.00, quantité: 2, totalPrice: 130 },
    {nom: 'Tente 3x3', item_type:'Acessoires', valeur:2500.00, contrib: 65.00, quantité: 2, totalPrice: 130 },
    {nom: 'Tente 3x3', item_type:'Acessoires', valeur:2500.00, contrib: 65.00, quantité: 2, totalPrice: 130 },
    {nom: 'Tente 3x3', item_type:'Acessoires', valeur:2500.00, contrib: 65.00, quantité: 2, totalPrice: 130 },
    {nom: 'Tente 3x3', item_type:'Acessoires', valeur:2500.00, contrib: 65.00, quantité: 2, totalPrice: 130 },
    {nom: 'Tente 3x3', item_type:'Acessoires', valeur:2500.00, contrib: 65.00, quantité: 2, totalPrice: 130 },
]);
</script>

<template>
    <div class="page">
        <header>
            <div class="infos">
                <img src="../assets/LOGO_SDC.png">
                <div class="general-info">
                    <p class="date">A Arvieux le 20/06/2025</p>
                    <div class="adress">
                        <p>Nom du Client</p>
                        <p>Adreses totale, 
                            possiblement sur plusieurs lignes 
                            Au moins 3</p>
                    </div>  
                </div>
            </div>
            <p class="context blue">Contribution mise à disposition de matériel Son et éclairage</p>
            <p class="nom-devis">Facture n°20250601</p>
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
                        <td>Technicien</td>
                        <td>330€</td>
                        <td>2</td>
                        <td>660€</td>
                    </tr>
                    <tr>
                        <td>Transport</td>
                        <td>0.70€</td>
                        <td>20</td>
                        <td>14€</td>
                    </tr>
                    <tr v-if="materiel.length < 10" class="Materiel">
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
                                        <th>Total</th>
                                    </tr>
                                </thead>
                                <tbody>
                                    <tr v-for="item in materiel">
                                        <td>{{ item.nom }}</td>
                                        <td>{{ item.item_type }}</td>
                                        <td>{{ item.valeur }}€</td>
                                        <td>{{ item.valeur * item.quantité }}€</td>
                                        <td>{{ item.contrib }}€</td>
                                        <td>{{ item.quantité }}</td>
                                        <td>{{ item.totalPrice }}€</td>
                                    </tr>
                                </tbody>
                                <tfoot class="blue">
                                    <tr>
                                        <td>TOTAL</td>
                                        <td></td>
                                        <td></td>
                                        <td>7854,26€</td>
                                        <td></td>
                                        <td></td>
                                        <td>1208,20€</td>
                                    </tr>
                                </tfoot>
                            </table>
                        </td>
                    </tr>
                    <tr v-else-if="materiel.length > 0">
                        <td>Matériel (détails page suivante)</td>
                        <td></td>
                        <td></td>
                        <td>10000,70€</td>
                    </tr>
                    <tr>
                        <td>Adhésion morale année scolaire 2025/2026</td>
                        <td></td>
                        <td></td>
                        <td>25€</td>
                    </tr>
                </tbody>
                <tfoot class="blue">
                    <tr>
                        <td>TOTAL Général</td>
                        <td></td>
                        <td></td>
                        <td>12080,20 €</td>
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
            <img src="../assets/QRCode.png">
        </footer>
    </div>

    <div class="page" v-if="materiel.length > 10">
        <header>
            <div class="infos">
                <img src="../assets/LOGO_SDC.png">
                <div class="general-info">
                    <p class="date">A Arvieux le 20/06/2025</p>
                    
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
                    <th>Total</th>
                </tr>
            </thead>
            <tbody>
                <tr v-for="item in materiel">
                    <td>{{ item.nom }}</td>
                    <td>{{ item.item_type }}</td>
                    <td>{{ item.valeur }}€</td>
                    <td>{{ item.valeur * item.quantité }}€</td>
                    <td>{{ item.contrib }}€</td>
                    <td>{{ item.quantité }}</td>
                    <td>{{ item.totalPrice }}€</td>
                </tr>
            </tbody>
            <tfoot class="blue">
                <tr>
                    <td>TOTAL</td>
                    <td></td>
                    <td></td>
                    <td>7854,26€</td>
                    <td></td>
                    <td></td>
                    <td>1208,20€</td>
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
            <img src="../assets/QRCode.png">
        </footer>
    </div>
</template>

<style scoped>
.page {
  width: 210mm;
  min-height: 297mm;

  margin-bottom: 1rem;
  padding: 20mm;
  padding-top: 15mm;
  padding-bottom: 10mm;

  color: black;
  background: white;
  box-sizing: border-box;
  page-break-after: always;
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
}

td {
    padding-left: 5px;
    padding-right: 5px;
    border: 1px solid deepskyblue;
}

th, tfoot {
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
    height: 100px;
}
</style>