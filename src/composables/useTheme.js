import { ref, onMounted, onBeforeUnmount} from 'vue';

const theme = ref('system');
const activeTheme = ref('light')
const mediaQuery = window.matchMedia('(prefers-color-scheme: dark)');
let mediaListener = () => applyTheme('system')

function applyTheme(mode) {
    const root = document.documentElement;
    root.classList.remove('theme-light','theme-dark');

    if(mode === 'light') {
        root.classList.add('theme-light');
        activeTheme.value = 'light';
    } else if (mode === 'dark') {
        root.classList.add('theme-dark');
        activeTheme.value = 'dark';
    } else {
        const prefersDark = window.matchMedia('(prefers-color-scheme: dark)').matches;
        root.classList.add(prefersDark ? 'theme-dark' : 'theme-light');
        activeTheme.value = prefersDark ? 'dark' : 'light';
    }
}

function setTheme(mode) {
    console.log('Setting theme to ' + mode);
    if(mode === 'system') {
        mediaQuery.addEventListener('change', mediaListener);
    } else {
        mediaQuery.removeEventListener('change', mediaListener);
    }

    theme.value = mode;
    localStorage.setItem('theme', mode);
    applyTheme(mode);
}

function loadTheme() {
    theme.value = localStorage.getItem('theme') || 'system';
    setTheme(theme.value);
}

export function useTheme() {
    onMounted(() => {
        loadTheme();
    });

    onBeforeUnmount(() => {
        mediaQuery.removeEventListener('change', mediaListener);
    });

    return {
        theme,
        activeTheme,
        setTheme,
        loadTheme
    }
}