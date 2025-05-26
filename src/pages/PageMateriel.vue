<script setup>
import { ref, onMounted, onUnmounted } from 'vue';
import ListeMateriel from '../components/ListeMateriel.vue';
import DisplayMateriel from '../components/DisplayMateriel.vue'

const wideWidth = 1080;

const isWide = ref(window.innerWidth > wideWidth);
function handleResize() {
    isWide.value = window.innerWidth > wideWidth;
}

onMounted(() => {
    window.addEventListener('resize', handleResize);
});

onUnmounted(() => {
    window.removeEventListener('resize', handleResize);
});

const display = ref(null)
function setDisplay(value) {
    display.value = value;
}
</script>

<template>
    <div class="wrapper" :class="{'item-display': display !== null, wide: isWide}">
        <ListeMateriel class="list" @display="setDisplay"/>
        <Transition name="slide">
            <DisplayMateriel 
                v-if="display !== null" 
                class="detail" 
                :item="display" 
                :setItem="setDisplay"
            />
        </Transition>
    </div>
</template>

<style scoped>
.wrapper {
    height: 100%;
    overflow: hidden;
    display: flex;
    flex-direction: column;
}

.wrapper.item-display.wide {
    flex-direction: row;
}
</style>