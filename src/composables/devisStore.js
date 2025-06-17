import { invoke } from '@tauri-apps/api/core';
import { defineStore } from 'pinia'
import { ref } from 'vue'

export const useDevisStore = defineStore('devis', () => {
    const devisInfos = ref({
        id: '',
        name: '',
        date: '',
        duration: 1,
    });

    const clientInfos = ref({
        name: '',
        eventName: '',
        adress: '',
        phone: '',
        mail: ''
    });

    const selectedItems = ref([]);
    const extraItems = ref([]);

    const utilitaries = ref({
        techQty: 0,
        techRate: 0,
        techHourly: false,
        transportKm: 0,
        transportRate: 0,
        membership: false,
        discountEuro: 0
    });

    function setItemQuantity(item, quantity, duration, price) {
        if ((quantity !== 'unset' && (!quantity || quantity <= 0)) || 
            (duration !== 'unset' && (!duration || duration <= 0))) {
            selectedItems.value = selectedItems.value.filter(i => i.id !== item.id);
            return;
        }
        const existing = selectedItems.value.find(i => i.id === item.id);

        if (existing) {
            if (quantity != 'unset') existing.quantity = quantity;
            if (duration != 'unset') existing.duration = duration;
            if (price != 'unset') existing.totalPrice = price;
        } else {
            const q = quantity !== 'unset' ? quantity : 1;
            const d = duration !== 'unset' ? duration : devisInfos.value.duration;

            selectedItems.value.push({
            ...item,
            quantity: q,
            duration: d,
            totalPrice: price ?? 0,
            });
        }
    }

    function saveDevis() {

    }

    function cancelDevis() {
        
    }

    return { devisInfos, clientInfos, selectedItems, 
        extraItems, utilitaries, setItemQuantity};
})
