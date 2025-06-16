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
        adhesion: false,
        discountPercent: 0,
        discountEuro: 0
    });

    function setItemQuantity(item, quantity) {
        const q = Math.max(0, Math.min(quantity, item.total))
        const existing = selectedItems.value.find(i => i.id === item.id);

        console.log("Item id : " + item.id + " Quandtity : " + quantity + " Existing : " + existing);

        if (q > 0) {
            if (existing) {
                existing.quantity = q;
            } else {
                selectedItems.value.push({
                ...item,
                quantity: q
                });
            }
        } else if (existing) {
            selectedItems.value = selectedItems.value.filter(i => i.id !== item.id);
        }
    }

    return { devisInfos, clientInfos, selectedItems, extraItems, utilitaries, setItemQuantity};
})
