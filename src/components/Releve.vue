<script setup>
const IBAN = 'FR76 ' + '\u200B' + '1680 ' + '\u200B' + '7001 ' + '\u200B' + '3131 ' + '\u200B' + '7631 ' + '\u200B' + '1521 ' + '\u200B' + '577';
</script>

<template>
<div class="rib">
    <h3>Relevé d'identité bancaire / Bank details statement</h3>
    <p>IBAN : <span>{{ IBAN }}</span></p>
    <p>BIC : <span>CCBPFRPPGRE</span></p>
    <p>Code Banque : <span>16807</span></p>
    <p>Code Guichet : <span>00131</span></p>
    <p>N° du compte : <span>31763115215</span></p>
    <p>Clé RIB : <span>77</span></p>
    <p>Domiciliation : <span>BPAURA GUILLESTRE</span></p>
</div>
</template>

<style scoped>
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
    font-size: 14px;
    padding-left: 5px;
    color: dimgray;
}
</style>