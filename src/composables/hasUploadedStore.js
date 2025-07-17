import { ref } from 'vue';

const hasUploaded = ref(false);
const syncError = ref('')

export function useHasUploaded() {
    function setHasUploaded(value) {
        hasUploaded.value = value;
    }

    function setSyncError(value) {
        syncError.value = value;
    }

    return { hasUploaded, syncError, setHasUploaded, setSyncError };
}