/* ================================================================
   soma-infra website — script.js
   Responsibilities:
   1. Scroll-triggered reveal animations
   2. Smooth scroll (for older browsers that ignore CSS)
   3. Nav shadow on scroll
   4. Copy-to-clipboard on code blocks
   No external dependencies.
================================================================ */

'use strict';

/* ── 1. SCROLL-TRIGGERED REVEALS ─────────────────────────────── */

function initReveal() {
  const prefersReduced = window.matchMedia('(prefers-reduced-motion: reduce)').matches;
  if (prefersReduced) {
    document.querySelectorAll('.reveal').forEach(el => el.classList.add('visible'));
    return;
  }

  const observer = new IntersectionObserver(
    (entries) => {
      entries.forEach(entry => {
        if (entry.isIntersecting) {
          entry.target.classList.add('visible');
          observer.unobserve(entry.target);
        }
      });
    },
    { threshold: 0.1, rootMargin: '0px 0px -40px 0px' }
  );

  document.querySelectorAll('.reveal').forEach(el => observer.observe(el));
}

/* ── 2. SMOOTH SCROLL (for older browsers that ignore CSS) ─────── */

function initSmoothScroll() {
  document.querySelectorAll('a[href^="#"]').forEach(link => {
    link.addEventListener('click', (e) => {
      const target = document.querySelector(link.getAttribute('href'));
      if (!target) return;
      e.preventDefault();
      target.scrollIntoView({ behavior: 'smooth', block: 'start' });
      // Move focus for accessibility
      target.setAttribute('tabindex', '-1');
      target.focus({ preventScroll: true });
    });
  });
}

/* ── 3. NAV SHADOW ON SCROLL ──────────────────────────────────── */

function initNavScroll() {
  const nav = document.querySelector('nav');
  if (!nav) return;

  window.addEventListener('scroll', () => {
    if (window.scrollY > 20) {
      nav.style.boxShadow = '0 1px 0 rgba(56,189,248,0.06), 0 8px 32px rgba(0,0,0,0.4)';
    } else {
      nav.style.boxShadow = 'none';
    }
  }, { passive: true });
}

/* ── 4. COPY-TO-CLIPBOARD ─────────────────────────────────────── */

function initCopyButtons() {
  document.querySelectorAll('.copy-btn').forEach(btn => {
    btn.addEventListener('click', () => {
      const targetId = btn.getAttribute('data-copy-target');
      const block = targetId ? document.getElementById(targetId) : btn.closest('.code-block-wrap')?.querySelector('.code-block');
      if (!block) return;

      // Extract plain text from the code block (skip line-number spans)
      const lines = block.querySelectorAll('.lc');
      const text = Array.from(lines).map(lc => lc.textContent).join('\n').trimEnd();

      navigator.clipboard.writeText(text).then(() => {
        btn.textContent = 'copied!';
        btn.classList.add('copied');
        setTimeout(() => {
          btn.textContent = 'copy';
          btn.classList.remove('copied');
        }, 1800);
      }).catch(() => {
        // Silent fail — clipboard not available in non-secure contexts
      });
    });
  });
}

/* ── INIT ─────────────────────────────────────────────────────── */

document.addEventListener('DOMContentLoaded', () => {
  initReveal();
  initSmoothScroll();
  initNavScroll();
  initCopyButtons();
});
