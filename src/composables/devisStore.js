import { defineStore } from 'pinia'
import { ref } from 'vue'

export const useDevisStore = defineStore('devis', () => {
    const selectedItems = ref([]);

    function setItemQuantity(item, quantity) {
        const q = Math.max(0, Math.min(quantity, item.total))
        const existing = selectedItems.value.find(i => i.id === item.id)

        if (q > 0) {
        if (existing) {
            existing.quantity = q
        } else {
            selectedItems.value.push({
            ...item,
            quantity: q
            })
        }
        } else if (existing) {
        selectedItems.value = selectedItems.value.filter(i => i.id !== item.id)
        }
    }

    const extraFields = ref({
        name: '',
        date: '',
        duration: 1,
        hourly: false,
        clientName: '',
        clientAdress: '',
        techQty: 0,
        techRate: 0,
        techHourly: false,
        transportKm: 0,
        transportRate: 0,
        adhesion: false,
        otherLabel: '',
        otherPrice: 0,
        discountPercent: 0,
        discountEuro: 0
    })

    return { selectedItems, setItemQuantity, extraFields }
})
