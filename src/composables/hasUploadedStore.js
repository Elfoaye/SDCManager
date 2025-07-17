import { ref } from 'vue';

const hasUploaded = ref(false);

export function useHasUploaded() {
    function setHasUploaded(value) {
        hasUploaded.value = value;
    }

    return { hasUploaded, setHasUploaded };
}