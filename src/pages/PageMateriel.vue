<script setup>
import { ref, computed, onMounted, onUnmounted, watch } from 'vue';
import ListeMateriel from '../components/ListeMateriel.vue';
import DisplayMateriel from '../components/DisplayMateriel.vue'
import ModifMateriel from '../components/ModifMateriel.vue'

const props = defineProps(['modif']);

const displayKey = ref(0);
const modifKey = ref(0);
const listKey = ref(0);

const display = ref(null);
const create = ref(false);
const listRef = ref(null);

const wideWidth = 1080;
const isWide = ref(window.innerWidth > wideWidth);

const transitionName = computed(() => {
  return isWide.value ?  'expand-height' : 'expand-width';
});

function setDisplay(value) {
    display.value = value;
    create.value = false;

    displayKey.value++;
    modifKey.value++;
}

function setCreate() {
    setDisplay(null);
    create.value = true;
}

async function onItemChange() {
    if(!listRef.value || !display.value) return;

    const newItem = await listRef.value.updateItem(display.value.id);
    setDisplay(newItem);
}

function onListChange() {
    listRef.value.updateData();
    onItemChange()
}

function handleResize() {
    isWide.value = window.innerWidth > wideWidth;
}

onMounted(() => {
    window.addEventListener('resize', handleResize);
});

onUnmounted(() => {
    window.removeEventListener('resize', handleResize);
});

watch(() => props.modif, () => {
    listKey.value++;
});
</script>

<template>
    <div class="wrapper" :class="{'item-display': display !== null, wide: isWide}">
        <ListeMateriel 
            class="list" 
            ref="listRef" 
            :key="listKey"
            :item="display" 
            :setItem="setDisplay"
            :setCreate="setCreate"
            :modif="modif"
        />
        <Transition :name="transitionName" mode="out-in">
            <ModifMateriel 
                v-if="(display !== null || create) && modif === true"
                class="detail" 
                :key="modifKey"
                :item="display" 
                :setItem="setDisplay"
                :create="create"
                @item-change="onListChange"
            />
            <DisplayMateriel 
                v-else-if="display !== null" 
                class="detail" 
                :key="displayKey"
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