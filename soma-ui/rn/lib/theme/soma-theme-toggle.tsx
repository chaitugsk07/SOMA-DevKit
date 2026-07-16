import { Pressable, Text } from 'react-native';
import { useTheme } from './theme-provider';

/** Light/dark switch for app headers. Self-contained (no component deps). */
export function SomaThemeToggle() {
  const { scheme, toggle } = useTheme();
  return (
    <Pressable
      onPress={toggle}
      accessibilityRole="switch"
      accessibilityLabel="Toggle color scheme"
      accessibilityState={{ checked: scheme === 'dark' }}
      className="h-9 w-9 items-center justify-center rounded-md border border-border bg-card active:opacity-70"
    >
      <Text className="text-base">{scheme === 'light' ? '🌙' : '☀️'}</Text>
    </Pressable>
  );
}
