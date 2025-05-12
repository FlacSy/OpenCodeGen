let currentLanguage = 'en';
let translations = {};

function setLanguage(lang) {
    currentLanguage = lang;
    loadTranslations();
}

function loadTranslations() {
    fetch(`/static/js/i18n/${currentLanguage}.json`)
        .then(response => response.text())
        .then(text => {
            try {
                translations = JSON.parse(text);
                updateContent();
            } catch (error) {
                console.error('Error parsing translations:', error);
            }
        })
        .catch(error => {
            console.error('Error loading translations:', error);
        });
}

function updateContent() {
    document.querySelectorAll('[data-i18n]').forEach(element => {
        const key = element.getAttribute('data-i18n');
        if (translations[key]) {
            element.textContent = translations[key];
        }
    });
}

document.addEventListener('DOMContentLoaded', () => {
    const savedLanguage = localStorage.getItem('language');
    if (savedLanguage) {
        setLanguage(savedLanguage);
    } else {
        setLanguage('en');
    }
}); 