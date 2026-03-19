const nav = document.getElementById('nav');
window.addEventListener('scroll', () => {
    nav.classList.toggle('scrolled', window.scrollY > 50);
}, { passive: true });

const io = new IntersectionObserver((entries) => {
    entries.forEach(e => { if (e.isIntersecting) { e.target.classList.add('visible'); io.unobserve(e.target); } });
}, { threshold: 0.08 });
document.querySelectorAll('.reveal').forEach(el => io.observe(el));

fetch('https://api.github.com/repos/Next-Tablet-Driver/NextTabletDriver/releases/latest')
    .then(r => r.json())
    .then(d => {
        if (d.tag_name) {
            const t = d.tag_name;
            const vt = document.getElementById('version-tag');
            const cv = document.getElementById('cta-version');
            if (vt) vt.innerHTML = `Latest release: <strong>${t}</strong>`;
            if (cv) cv.textContent = t;
        }
    })
    .catch(() => {
        const vt = document.getElementById('version-tag');
        if (vt) vt.innerHTML = `Latest release: <strong>v1.26.1903.03</strong>`;
    });