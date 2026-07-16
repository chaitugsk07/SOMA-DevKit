import { useState, type ReactNode } from 'react';
import { View, Pressable } from 'react-native';
import { Text } from '@/lib/components';
import { BreakpointProvider } from '@/lib/hooks';
import { cn } from '@/lib/utils/cn';

/** Device preview widths. `null` = full-bleed (desktop). */
const MODES = [
  { key: 'phone', label: 'Phone', width: 420 },
  { key: 'tablet', label: 'Tablet', width: 834 },
  { key: 'desktop', label: 'Desktop', width: null },
] as const;
type ModeKey = (typeof MODES)[number]['key'];

/**
 * Frames an app-style screen at a selectable device width (Phone 420 / Tablet 834 /
 * Desktop full-bleed) so responsive layouts are actually visible in the playground.
 * A screen's own `sm:`/`md:`/`lg:` classes and useBreakpoint() react to the framed
 * width because the frame constrains real layout width, not just a visual scale.
 */
export function PhoneFrame({ children }: { children: ReactNode }) {
  const [mode, setMode] = useState<ModeKey>('phone');
  const width = MODES.find((m) => m.key === mode)!.width;

  return (
    <View className="flex-1 bg-background">
      <View className="flex-row items-center justify-center gap-1 border-b border-border bg-card px-2 py-2">
        {MODES.map((m) => (
          <Pressable
            key={m.key}
            onPress={() => setMode(m.key)}
            accessibilityRole="button"
            accessibilityState={{ selected: mode === m.key }}
            className={cn(
              'rounded-md px-3 py-1.5 active:opacity-70',
              mode === m.key ? 'bg-primary' : 'bg-transparent',
            )}
          >
            <Text
              className={cn(
                'text-xs font-heading-medium',
                mode === m.key ? 'text-primary-foreground' : 'text-muted-foreground',
              )}
            >
              {m.label}
            </Text>
          </Pressable>
        ))}
      </View>

      <View className="flex-1 items-center">
        <View
          className={cn('flex-1 overflow-hidden', width !== null && 'border-x border-border')}
          style={{ width: width ?? '100%', maxWidth: '100%' }}
        >
          {/* Provider measures this framed width so screens' useBreakpoint()/<Columns>
              react to the device mode, not the browser window. `key` remounts the
              provider on mode change so it re-measures cleanly. */}
          <BreakpointProvider key={mode}>{children}</BreakpointProvider>
        </View>
      </View>
    </View>
  );
}
