# soma-ui — React Native

The React Native target of soma-ui: a shadcn-style, copy-paste/you-own-the-source
component library in soma-ui's Palantir design language (slate/blue, light + dark,
Outfit body + Rajdhani headings). Styled with [NativeWind](https://www.nativewind.dev)
(Tailwind for RN). Self-contained — does not depend on the `web/` or `flutter/` targets.

## Run the playground

```sh
cd rn
npm install
npx expo start        # then press i (iOS), a (Android), or w (web)
```

## Verify

```sh
npx tsc --noEmit      # typecheck
npx expo-doctor       # dependency / config health
```

## Structure (mirrors web/ and flutter/)

```
lib/
  theme/        SomaThemeProvider, useTheme, SomaThemeToggle, tokens (light/dark), fonts
  utils/        cn()
  components/
    inputs/         Button, Input, Switch, Checkbox, Textarea
    data-display/   Text, Badge, Avatar, Card (+ Header/Title/Description/Content/Footer)
    feedback/       Alert, Skeleton, Spinner, Progress
    layout/         Separator
    overlays/       Dialog
playground/     gallery app (home + one preview screen per component, light/dark toggle)
App.tsx         loads fonts, wraps in SomaThemeProvider, renders the gallery
```

Public API is flat — import everything from the barrel:

```tsx
import { Button, Card, useTheme } from '@/lib/components'; // or '@/lib'
```

## Tokens

Ported verbatim from `../web/theme/tokens.css` into `lib/theme/tokens.ts` (HSL triplets),
applied per-theme via NativeWind `vars()` and resolved through `hsl(var(--x))` in
`tailwind.config.js`. Keep these in sync with the web source of truth when it changes.

## Fonts

Outfit + Rajdhani TTFs live in `assets/fonts/` (copied from the flutter target) and are
loaded with `expo-font`. Tailwind families: `font-body` (Outfit), `font-heading` (Rajdhani),
plus weighted variants (`font-body-medium`, `font-heading-semibold`, …).
