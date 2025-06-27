<script setup>
defineProps({
    writeDate: String,
    pageNumber: Number,
});
</script>

<template>
    <div class="page">
        <header>
        <div class="infos">
            <img src="../assets/LOGO_SDC.png" />
            <div class="general-info">
            <p class="date">A Arvieux le {{ writeDate }}</p>
            </div>
        </div>
        </header>

        <slot></slot>

        <footer>
        <p class="legal">
            Son des Cimes – 39 Imp. Du Pellas – 05350 Arvieux<br />
            Association loi 1901 reconnue d’intérêt général n° W051000990 - Code APE : 9329Z<br />
            Siret : 513 386 979 000 19 - ☎ : 06 62 54 34 79 <br />
            sondescimes@gmail.com - http://sondescimes.lescigales.org<br />
            Non assujettie à la T.V.A.<br />
        </p>
        <div class="number">
            <img src="../assets/QRCode.png" />
            <p>{{ pageNumber }}</p>
        </div>
        </footer>
    </div>
</template>

