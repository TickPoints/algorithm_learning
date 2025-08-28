document.addEventListener('DOMContentLoaded', () => {
    const darkThemes = ['ayu', 'navy', 'coal'];
    const lightThemes = ['light', 'rust'];
    const htmlClass = document.documentElement.classList;

    const isDarkTheme = darkThemes.some(theme => htmlClass.contains(theme));
    
    mermaid.initialize({
        theme: isDarkTheme ? 'dark' : 'default',
        startOnLoad: true,
        experimental: { xyChart: true }
    });

    const handleThemeChange = (isLight) => {
        if (isLight !== !isDarkTheme) {
            mermaid.initialize({ theme: isLight ? 'default' : 'dark' });
            mermaid.run();
        }
    };

    darkThemes.forEach(id => {
        document.getElementById(id)?.addEventListener('click', () => handleThemeChange(false));
    });
    lightThemes.forEach(id => {
        document.getElementById(id)?.addEventListener('click', () => handleThemeChange(true));
    });
});