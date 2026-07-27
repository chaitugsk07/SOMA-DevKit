/** @type {import('tailwindcss').Config} */
module.exports = {
  darkMode: "class",
  // ── Consumer safelist ──────────────────────────────────────────────────────
  // build-css.sh scans ONLY soma-ui/src/ (SOMA-DevKit is a submodule shared
  // across multiple suites; its build must never reference a consumer's paths).
  // Tailwind classes used solely in a consuming suite's files (e.g.
  // SOMA-Control dashboards/) are silently absent from the output because
  // Tailwind only emits classes it sees in the content scan.
  //
  // Add entries here — grouped by category — when a class family is genuinely
  // needed by consumers but not present in soma-ui component sources.
  // Use patterns for families (avoids listing every specific value); use
  // explicit strings for one-offs and arbitrary values.
  //
  // When adding a new dashboard class that renders invisibly, search here first
  // before re-running build-css.sh, then add the entry and regenerate.
  safelist: [
    // Sizing — skeleton placeholders, table column widths, dropdown min-widths
    { pattern: /^(w|h)-(3|16|20|24|28|32|36|40|52|56|fit)$/ },
    { pattern: /^min-w-(24|28|32|40|48)$/ },
    'min-h-32', 'max-w-md',
    'max-h-[400px]', 'max-h-[500px]', 'max-w-[200px]',

    // Spacing — margin/padding combos used in dashboard page layouts
    { pattern: /^(ml|mt|mx|my)-(1|2|4|5|12)$/ },
    { pattern: /^(pb|pl|pr|pt)-(0|1|2|3|4)$/ },
    'py-8', 'space-y-5', 'space-y-10',

    // Grid — breakpoint/column-count combos beyond what soma-ui uses internally
    'col-span-2',
    'lg:grid-cols-2', 'lg:grid-cols-5',
    'sm:grid-cols-3', 'sm:grid-cols-4',

    // Named-color status indicators (emerald/green = success, amber = warning)
    // soma-ui uses semantic CSS-var tokens; dashboards use named colors for
    // status badges, banners, and inline success/error states.
    // Listed explicitly (not as a broad pattern) to avoid generating CSS for
    // every color × shade × opacity-modifier combination.
    'bg-amber-500', 'bg-emerald-50', 'bg-emerald-100', 'bg-green-50', 'bg-green-500/10',
    'dark:bg-amber-600', 'dark:bg-emerald-900', 'dark:bg-emerald-900/20',
    'dark:bg-emerald-950', 'dark:bg-green-950',
    'border-emerald-400', 'border-emerald-400/40', 'border-emerald-500',
    'border-green-200', 'border-green-500/30', 'border-green-600',
    'dark:border-green-800',
    'text-blue-600',
    'text-emerald-600', 'text-emerald-700', 'text-emerald-800',
    'text-green-600', 'text-green-700', 'text-green-800',
    'dark:text-emerald-200', 'dark:text-emerald-300', 'dark:text-emerald-400',
    'dark:text-green-200', 'dark:text-green-400',
    'text-white',

    // Opacity-modified semantic colors (subtle backgrounds, tinted borders)
    'border-destructive/30', 'border-destructive/40', 'border-success/20',
    'text-muted-foreground/40',

    // Hover states (row highlights, link colours in dashboard tables/lists)
    'hover:bg-muted/30', 'hover:border-border',
    'hover:text-destructive', 'hover:text-muted-foreground', 'hover:text-primary',

    // Focus ring (raw <input>/<textarea> elements built directly in dashboard pages)
    'focus:ring-2', 'focus:ring-ring',

    // Structural pseudo-variants
    'first:border-t-0',

    // Miscellaneous utilities used in dashboard pages
    'break-all', 'opacity-80', 'resize-y', 'right-0',
    'ring-offset-background', 'select-all', 'text-ellipsis',
    'text-right', 'tracking-wide',

    // Arbitrary values
    'text-[0.7rem]', 'tracking-[0.14em]',
  ],
  theme: {
    extend: {
      colors: {
        background: "hsl(var(--background) / <alpha-value>)",
        foreground: "hsl(var(--foreground) / <alpha-value>)",
        card: {
          DEFAULT: "hsl(var(--card) / <alpha-value>)",
          foreground: "hsl(var(--card-foreground) / <alpha-value>)",
        },
        muted: {
          DEFAULT: "hsl(var(--muted) / <alpha-value>)",
          foreground: "hsl(var(--muted-foreground) / <alpha-value>)",
        },
        primary: {
          DEFAULT: "hsl(var(--primary) / <alpha-value>)",
          foreground: "hsl(var(--primary-foreground) / <alpha-value>)",
        },
        secondary: {
          DEFAULT: "hsl(var(--secondary) / <alpha-value>)",
          foreground: "hsl(var(--secondary-foreground) / <alpha-value>)",
        },
        accent: {
          DEFAULT: "hsl(var(--accent) / <alpha-value>)",
          foreground: "hsl(var(--accent-foreground) / <alpha-value>)",
        },
        destructive: {
          DEFAULT: "hsl(var(--destructive) / <alpha-value>)",
          foreground: "hsl(var(--destructive-foreground) / <alpha-value>)",
        },
        success: {
          DEFAULT: "hsl(var(--success) / <alpha-value>)",
          foreground: "hsl(var(--success-foreground) / <alpha-value>)",
        },
        warning: {
          DEFAULT: "hsl(var(--warning) / <alpha-value>)",
          foreground: "hsl(var(--warning-foreground) / <alpha-value>)",
        },
        info: {
          DEFAULT: "hsl(var(--info) / <alpha-value>)",
          foreground: "hsl(var(--info-foreground) / <alpha-value>)",
        },
        contrast: {
          DEFAULT: "hsl(var(--contrast) / <alpha-value>)",
          foreground: "hsl(var(--contrast-foreground) / <alpha-value>)",
        },
        border: "hsl(var(--border) / <alpha-value>)",
        input: "hsl(var(--input) / <alpha-value>)",
        ring: "hsl(var(--ring) / <alpha-value>)",
      },
      boxShadow: {
        'elev-sm': 'var(--shadow-sm)',
        'elev': 'var(--shadow)',
        'elev-md': 'var(--shadow-md)',
        'elev-lg': 'var(--shadow-lg)',
      },
      fontFamily: {
        sans: ["var(--font-body)", "ui-sans-serif", "system-ui", "sans-serif"],
        heading: ["var(--font-heading)", "ui-sans-serif", "system-ui", "sans-serif"],
      },
      fontSize: {
        xs:   ['0.75rem',    { lineHeight: '1.5' }],
        sm:   ['0.8125rem',  { lineHeight: '1.5' }],
        base: ['0.875rem',   { lineHeight: '1.5' }],
        lg:   ['1rem',       { lineHeight: '1.4' }],
        xl:   ['1.125rem',   { lineHeight: '1.35', letterSpacing: '-0.01em' }],
        '2xl':['1.25rem',    { lineHeight: '1.3',  letterSpacing: '-0.01em' }],
        '3xl':['1.5rem',     { lineHeight: '1.25', letterSpacing: '-0.01em' }],
      },
      keyframes: {
        fadeIn: { "0%": { opacity: "0" }, "100%": { opacity: "1" } },
        slideUp: { "0%": { opacity: "0", transform: "translateY(16px)" }, "100%": { opacity: "1", transform: "translateY(0)" } },
        slideDown: { "0%": { opacity: "0", transform: "translateY(-16px)" }, "100%": { opacity: "1", transform: "translateY(0)" } },
        slideLeft: { "0%": { opacity: "0", transform: "translateX(16px)" }, "100%": { opacity: "1", transform: "translateX(0)" } },
        slideRight: { "0%": { opacity: "0", transform: "translateX(-16px)" }, "100%": { opacity: "1", transform: "translateX(0)" } },
        scaleIn: { "0%": { opacity: "0", transform: "scale(0.9)" }, "100%": { opacity: "1", transform: "scale(1)" } },
        bounceIn: { "0%": { opacity: "0", transform: "scale(0.3)" }, "50%": { opacity: "1", transform: "scale(1.05)" }, "70%": { transform: "scale(0.9)" }, "100%": { opacity: "1", transform: "scale(1)" } },
        pulseSoft: { "0%, 100%": { opacity: "1" }, "50%": { opacity: "0.5" } },
        marquee: { from: { transform: "translateX(0)" }, to: { transform: "translateX(-50%)" } },
        shimmer: { from: { backgroundPosition: "200% 0" }, to: { backgroundPosition: "-200% 0" } },
      },
      animation: {
        "fade-in": "fadeIn 0.4s ease-out forwards",
        "slide-up": "slideUp 0.4s ease-out forwards",
        "slide-down": "slideDown 0.4s ease-out forwards",
        "slide-left": "slideLeft 0.4s ease-out forwards",
        "slide-right": "slideRight 0.4s ease-out forwards",
        "scale-in": "scaleIn 0.3s ease-out forwards",
        "bounce-in": "bounceIn 0.5s ease-out forwards",
        "pulse-soft": "pulseSoft 2s ease-in-out infinite",
        "marquee": "marquee 20s linear infinite",
        "shimmer": "shimmer 2s linear infinite",
      },
    },
  },
};
