<script setup>
import { ref, onMounted, onBeforeUnmount, watch } from 'vue';
import ListeMateriel from '../components/ListeMateriel.vue';
import DisplayMateriel from '../components/DisplayMateriel.vue'
import ModifMateriel from '../components/ModifMateriel.vue'

const props = defineProps(['modif','setDocument']);

const displayKey = ref(0);
const modifKey = ref(0);
const listKey = ref(0);

const display = ref(null);
const create = ref(false);
const listRef = ref(null);

const transitionName = ref('expand-height');

const updateTransition = () => {
  transitionName.value = window.matchMedia('(min-width: 1024px)').matches 
    ? 'expand-width' 
    : 'expand-height';
};

function setDisplay(value) {
    if(display.value === value) {
        display.value = null;
    } else {
        display.value = value;
    }
    
    create.value = false;

    displayKey.value++;
    modifKey.value++;
}

async function refreshDisplay(value) {
    await listRef.value.updateData();

    if(!value) {
        setDisplay(null);
        return;
    }
    const newItem = await listRef.value.updateItem(value);
    setDisplay(newItem);
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

onMounted(() => {
  updateTransition();
  window.addEventListener('resize', updateTransition);
});

onBeforeUnmount(() => {
  window.removeEventListener('resize', updateTransition);
});

watch(() => props.modif, () => {
    listKey.value++;
});
</script>

<template>
    <div class="wrapper" :class="{'item-display': display !== null}">
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
                :setItemRefresh="refreshDisplay"
                :create="create"
                @item-change="onItemChange"
            />
            <DisplayMateriel 
                v-else-if="display !== null" 
                class="detail" 
                :key="displayKey"
                :item="display" 
                :setItem="setDisplay"
                :setDocument="props.setDocument"
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
    justify-content: center;
}

.list {
    transition: width 0.3s ease, height 0.3s ease;
}

/* Transition verticale */
.expand-height-enter-active,
.expand-height-leave-active {
  transition: transform 0.3s ease, opacity 0.3s ease;
  will-change: transform, opacity;
}
.expand-height-enter-from,
.expand-height-leave-to {
  opacity: 0;
  transform: scaleY(0);
  transform-origin: bottom;
}
.expand-height-enter-to,
.expand-height-leave-from {
  opacity: 1;
  transform: scaleY(1);
  transform-origin: bottom;
}

/* Transition horizontale */
.expand-width-enter-active,
.expand-width-leave-active {
  transition: transform 0.3s ease, opacity 0.3s ease;
  will-change: transform, opacity;
}
.expand-width-enter-from,
.expand-width-leave-to {
  opacity: 0;
  transform: scaleX(0);
  transform-origin: left;
}
.expand-width-enter-to,
.expand-width-leave-from {
  opacity: 1;
  transform: scaleX(1);
  transform-origin: left;
}

@media (min-width: 1024px) {
    .wrapper {
        flex-direction: row;
    }

    .detail {
        min-width: 23rem;
        max-width: 30rem;
    }
}
</style>