import { invoke } from '@tauri-apps/api/core';
import { defineStore } from 'pinia'
import { ref, computed } from 'vue'

export const useDevisStore = defineStore('devis', () => {
    const formulas = ref(null);
    invoke('get_loc_formulas').then((data) => formulas.value = data);

    const clients = ref([]);
    invoke('get_client_infos').then((data) => clients.value = data);

    const devisInfos = ref({
        id: 0,
        name: '',
        date: '',
        writeDate: '',
        duration: 1,
        type: ''
    });

    const clientInfos = ref({
        id: 0,
        name: '',
        eventName: '',
        adress: '',
        phone: '',
        mail: ''
    });

    const utilitaries = ref({
        techQty: 0,
        techRate: 0,
        techHourly: false,
        transportKm: 0,
        transportRate: 0,
        membership: false,
        discountEuro: 0
    });

    const selectedItems = ref([]);
    const extraItems = ref([]);

    const isFacture = computed(() => devisInfos.value.type.includes('facture'));

    function priceLoc(item, quantity, duration) {
        if(!item || !formulas.value || quantity <= 0 || duration <= 0) return 0;

        return quantity * (item.contrib + (duration - 1) * (item.contrib * formulas.value.contrib_following));
    }

    function setItem(item, quantity, duration) {
        if ((quantity !== 'unset' && (!quantity || quantity <= 0)) || 
            (duration !== 'unset' && (!duration || duration <= 0))) {
            selectedItems.value = selectedItems.value.filter(i => i.id !== item.id);
            return;
        }
        const existing = selectedItems.value.find(i => i.id === item.id);

        if (existing) {
            if (quantity != 'unset') existing.quantity = quantity;
            if (duration != 'unset') existing.duration = duration;
            existing.totalPrice = priceLoc(item, existing.quantity, existing.duration);
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

    async function saveDevis(state) {
        if(isFacture.value) return { result: 'error', message: 'Impossible de sauvegarder une facture en temps que devis' };

        const today = new Date();

        const day = String(today.getDate()).padStart(2, '0');
        const month = String(today.getMonth() + 1).padStart(2, '0'); // Les mois commencent à 0
        const year = today.getFullYear();

        devisInfos.value.type = state;

        const fullDocument = {
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
                date_crea: `${day}/${month}/${year}`,
                durée: devisInfos.value.duration,
                nb_tech: utilitaries.value.techQty,
                taux_tech: utilitaries.value.techRate,
                nb_km: utilitaries.value.transportKm,
                taux_km: utilitaries.value.transportRate,
                adhesion: utilitaries.value.membership,
                promo: utilitaries.value.discountEuro,
                etat: state
            },
            items: selectedItems.value.map(item => ({
                item: {
                    id: item.id,
                    nom: item.nom,
                    item_type: item.item_type,
                    total: item.total,
                    dispo: item.dispo,
                    valeur: item.valeur,
                    contrib: item.contrib,
                    nb_sorties: item.nb_sorties,
                    benef: item.benef
                },
                quantité: item.quantity,
                durée: item.duration,
                etat: state
            })),
            extra: extraItems.value.map(extra => ({
                id: 0,
                devis_id: devisInfos.value.id,
                nom: extra.name,
                prix: parseFloat(extra.price)
            }))
        };

        try {
            devisInfos.value.id = await invoke('save_devis', { fullDocument: fullDocument });
            clients.value = await invoke('get_client_infos');
            return { result: 'success', message: "Devis sauvegardé" };
        } catch (err) {
            return { result: 'error', message: err.toString() };
        }
    }

    async function loadDocument(document) {
        try {
            const fullDocument = document.facture ? await invoke('load_facture', { factureId: document.id }) :
            await invoke('load_devis', { devisId: document.id });

            clientInfos.value = {
                name: fullDocument.client.nom,
                eventName: fullDocument.client.evenement,
                adress: fullDocument.client.adresse,
                phone: fullDocument.client.tel,
                mail: fullDocument.client.mail,
                id: fullDocument.client.id
            };

            devisInfos.value = {
                id: fullDocument.devis.id,
                name: fullDocument.devis.nom,
                date: fullDocument.devis.date,
                writeDate: fullDocument.devis.date_crea,
                duration: fullDocument.devis.durée,
                type: fullDocument.devis.etat
            };

            selectedItems.value = fullDocument.items.map(fullItem => ({
                ...fullItem.item,
                quantity: fullItem.quantité,
                duration: fullItem.durée,
                state: fullItem.etat,
                totalPrice: priceLoc(fullItem.item, fullItem.quantité, fullItem.durée)
            }));

            extraItems.value = fullDocument.extra.map(extra => ({
                name: extra.nom,
                price: extra.prix
            }));

            utilitaries.value = {
                techQty: fullDocument.devis.nb_tech,
                techRate: fullDocument.devis.taux_tech,
                techHourly: (fullDocument.devis.taux_tech < 100 && fullDocument.taux_tech > 0),
                transportKm: fullDocument.devis.nb_km,
                transportRate: fullDocument.devis.taux_km,
                membership: fullDocument.devis.adhesion,
                discountEuro: fullDocument.devis.promo
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
            type: ''
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

    return { formulas, clients, devisInfos, clientInfos, selectedItems, 
        extraItems, utilitaries, isFacture, setItem, saveDevis, loadDocument, reset};
});
