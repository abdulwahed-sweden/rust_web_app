console.log("Rust Web App is running!");

// Add a simple animation to the header
document.addEventListener('DOMContentLoaded', function() {
    const h1 = document.querySelector('h1');
    if (h1) {
        h1.style.opacity = 0;
        let opacity = 0;
        const interval = setInterval(() => {
            opacity += 0.1;
            h1.style.opacity = opacity;
            if (opacity >= 1) clearInterval(interval);
        }, 100);
    }
});