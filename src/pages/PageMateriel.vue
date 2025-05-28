<script setup>
import { ref, computed, onMounted, onUnmounted, nextTick } from 'vue';
import ListeMateriel from '../components/ListeMateriel.vue';
import DisplayMateriel from '../components/DisplayMateriel.vue'

const componentKey = ref(0);

const display = ref(null)
function setDisplay(value) {
    display.value = value;
    componentKey.value++;
}

const listRef = ref(null);
async function onItemChange() {
    if(!listRef.value) return;

    const newItem = await listRef.value.updateItem(display.value.id);
    setDisplay(newItem);
}

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

const transitionName = computed(() => {
  return isWide.value ?  'expand-height' : 'expand-width';
});

</script>

<template>
    <div class="wrapper" :class="{'item-display': display !== null, wide: isWide}">
        <ListeMateriel class="list" ref="listRef" @display="setDisplay"/>
        <Transition :name="transitionName" mode="out-in">
            <DisplayMateriel 
                v-if="display !== null" 
                :key="componentKey"
                class="detail" 
                :item="display" 
                :setItem="setDisplay"
                @item-change="onItemChange"
            />
        </Transition>
    </div>
</template>

<style scoped>
.wrapper {
    height: 100%;
    width: 100%;
    overflow: hidden;
    display: flex;
    flex-direction: column;
}

.wrapper.wide {
    flex-direction: row;
}

.list {
    transition: width 0.3s ease, height 0.3s ease;
}

.expand-height-enter-from,
.expand-height-leave-to {
    opacity: 0;
    transform: scaleX(0);
    transform-origin: left;
}
.expand-height-enter-to, 
.expand-height-leave-from {
    opacity: 1;
    transform: scaleX(1);
    transform-origin: left;
}
.expand-height-enter-active,
.expand-height-leave-active {
    transition: transform 0.3s ease, opacity 0.3s ease;
}

.expand-width-enter-from,
.expand-width-leave-to {
    opacity: 0;
    transform: scaleY(0);
    transform-origin: bottom;
}
.expand-width-enter-to,
.expand-width-leave-from {
    opacity: 1;
    transform: scaleY(1);
    transform-origin: bottom;
}
.expand-width-enter-active,
.expand-width-leave-active {
    transition: transform 0.3s ease, opacity 0.3s ease;
}


</style>