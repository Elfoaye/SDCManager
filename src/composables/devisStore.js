import { invoke } from '@tauri-apps/api/core';
import { defineStore } from 'pinia'
import { ref } from 'vue'

export const useDevisStore = defineStore('devis', () => {
    const formulas = ref(null);
    invoke('get_loc_formulas').then((data) => formulas.value = data);

    const devisInfos = ref({
        id: 0,
        name: '',
        date: '',
        duration: 1,
    });

    const clientInfos = ref({
        id: 0,
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

    function priceLoc(item, quantity, duration) {
        if(!item || !formulas.value || quantity <= 0 || duration <= 0) return 0;

        return quantity * (item.contrib + (duration - 1) * (item.contrib * formulas.value.contrib_following));
    }

    function setItemQuantity(item, quantity, duration) {
        if ((quantity !== 'unset' && (!quantity || quantity <= 0)) || 
            (duration !== 'unset' && (!duration || duration <= 0))) {
            selectedItems.value = selectedItems.value.filter(i => i.id !== item.id);
            return;
        }
        const existing = selectedItems.value.find(i => i.id === item.id);

        if (existing) {
            if (quantity != 'unset') existing.quantity = quantity;
            if (duration != 'unset') existing.duration = duration;
            totalPrice = priceLoc(item, existing.quantity, existing.duration);
        } else {
            const q = quantity !== 'unset' ? quantity : 1;
            const d = duration !== 'unset' ? duration : devisInfos.value.duration;

            selectedItems.value.push({
            ...item,
            quantity: q,
            duration: d,
            totalPrice: priceLoc(item, q, d)
            });
        }
    }

    async function saveDevis() {
        const fullDevis = {
            client: {
                id: clientInfos.value.id,
                nom: clientInfos.value.name,
                evenement: clientInfos.value.eventName,
                adresse: clientInfos.value.adress,
                tel: clientInfos.value.phone,
                mail: clientInfos.value.mail
            },
            devis: {
                id: devisInfos.value.id,
                client_id: clientInfos.value.id,
                nom: devisInfos.value.name,
                date: devisInfos.value.date,
                durée: devisInfos.value.duration,
                nb_tech: utilitaries.value.techQty,
                taux_tech: utilitaries.value.techRate,
                nb_km: utilitaries.value.transportKm,
                taux_km: utilitaries.value.transportRate,
                adhesion: utilitaries.value.membership,
                promo: utilitaries.value.discountEuro,
                etat: "devis"
            },
            items: selectedItems.value.map(item => ({
                id: 0,
                devis_id: devisInfos.value.id,
                item_id: item.id,
                quantité: item.quantity,
                durée: item.duration,
                etat: "devis"
            })),
            extra: extraItems.value.map(extra => ({
                id: 0,
                devis_id: devisInfos.value.id,
                nom: extra.name,
                prix: parseFloat(extra.price)
            }))
        };

        try {
            devisInfos.value.id = await invoke('save_devis', { fullDevis: fullDevis });
            return { result: 'success', message: "Devis sauvegardé" };
        } catch (err) {
            return { result: 'error', message: err.toString() };
        }
    }

    async function loadDevis(id) {
        try {
            const fullDevis = await invoke('load_devis', { devisId: id });

            clientInfos.value = {
                name: fullDevis.client.nom,
                eventName: fullDevis.client.evenement,
                adress: fullDevis.client.adresse,
                phone: fullDevis.client.tel,
                mail: fullDevis.client.mail,
                id: fullDevis.client.id
            };

            devisInfos.value = {
                id: fullDevis.devis.id,
                name: fullDevis.devis.nom,
                date: fullDevis.devis.date,
                duration: fullDevis.devis.durée
            };

            selectedItems.value = fullDevis.items.map(fullItem => ({
                ...fullItem.item,
                quantity: fullItem.quantité,
                duration: fullItem.durée,
                state: fullItem.etat,
                totalPrice: priceLoc(fullItem.item, fullItem.quantité, fullItem.durée)
            }));

            extraItems.value = fullDevis.extra.map(extra => ({
                name: extra.nom,
                price: extra.prix
            }));

            utilitaries.value = {
                techQty: fullDevis.devis.nb_tech,
                techRate: fullDevis.devis.taux_tech,
                techHourly: (fullDevis.devis.taux_tech < 100),
                transportKm: fullDevis.devis.nb_km,
                transportRate: fullDevis.devis.taux_km,
                membership: fullDevis.devis.adhesion,
                discountEuro: fullDevis.devis.promo
            };         
        } catch (err) {
            throw err;
        }
    }

    function reset() {
        devisInfos.value = {
            id: 0,
            name: '',
            date: '',
            duration: 1,
        };
        clientInfos.value = {
            id: 0,
            name: '',
            eventName: '',
            adress: '',
            phone: '',
            mail: ''
        };
        selectedItems.value = [];
        extraItems.value = [];
        utilitaries.value = { 
            techQty: 0,
            techRate: 0,
            techHourly: false,
            transportKm: 0,
            transportRate: 0,
            membership: false,
            discountEuro: 0 
        };
    }

    return { devisInfos, clientInfos, selectedItems, 
        extraItems, utilitaries, setItemQuantity, saveDevis, loadDevis, reset};
});
