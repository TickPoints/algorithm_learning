document.addEventListener('DOMContentLoaded', () => {
    // 兼容 catppuccin
    const darkThemes = ['ayu', 'navy', 'coal', 'frappe', 'mocha', 'macchiato'];
    const lightThemes = ['light', 'rust', 'latte'];
    
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