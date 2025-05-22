import { ref } from 'vue';

const breadcrumb = ref([]);

export function useBreadcrumb() {
    function setBreadcrumb(items) {
        breadcrumb.value = items;
    }

    return { breadcrumb, setBreadcrumb };
}