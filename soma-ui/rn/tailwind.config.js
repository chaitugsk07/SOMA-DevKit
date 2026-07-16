/** @type {import('tailwindcss').Config} */
// Palantir tokens ported from soma-ui web/theme/tokens.css.
// Colors resolve from CSS vars set per-theme by lib/theme (NativeWind vars()).
module.exports = {
  presets: [require('nativewind/preset')],
  content: ['./App.tsx', './lib/**/*.{ts,tsx}', './playground/**/*.{ts,tsx}'],
  theme: {
    extend: {
      colors: {
        background: 'hsl(var(--background))',
        foreground: 'hsl(var(--foreground))',
        card: {
          DEFAULT: 'hsl(var(--card))',
          foreground: 'hsl(var(--card-foreground))',
        },
        muted: {
          DEFAULT: 'hsl(var(--muted))',
          foreground: 'hsl(var(--muted-foreground))',
        },
        primary: {
          DEFAULT: 'hsl(var(--primary))',
          foreground: 'hsl(var(--primary-foreground))',
        },
        secondary: {
          DEFAULT: 'hsl(var(--secondary))',
          foreground: 'hsl(var(--secondary-foreground))',
        },
        accent: {
          DEFAULT: 'hsl(var(--accent))',
          foreground: 'hsl(var(--accent-foreground))',
        },
        destructive: {
          DEFAULT: 'hsl(var(--destructive))',
          foreground: 'hsl(var(--destructive-foreground))',
        },
        success: {
          DEFAULT: 'hsl(var(--success))',
          foreground: 'hsl(var(--success-foreground))',
        },
        border: 'hsl(var(--border))',
        input: 'hsl(var(--input))',
        ring: 'hsl(var(--ring))',
      },
      borderRadius: {
        sm: '4px',
        md: '6px',
        lg: '8px',
      },
      fontFamily: {
        body: ['Outfit'],
        'body-medium': ['Outfit-Medium'],
        'body-semibold': ['Outfit-SemiBold'],
        'body-bold': ['Outfit-Bold'],
        heading: ['Rajdhani'],
        'heading-medium': ['Rajdhani-Medium'],
        'heading-semibold': ['Rajdhani-SemiBold'],
        'heading-bold': ['Rajdhani-Bold'],
        // ponytail: platform serif for s-delights PullQuote/DidYouKnow. Upgrade
        // path: bundle Lora/Libre-Baskerville TTF + expo-font for print parity.
        serif: ['Georgia', 'ui-serif', 'serif'],
      },
    },
  },
  plugins: [],
};
