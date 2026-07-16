import { useState, type ReactNode } from 'react';
import { View, Pressable } from 'react-native';
import { MotiView } from 'moti';
import { Text } from '@/lib/components/data-display/text';
import { cn } from '@/lib/utils/cn';

export type TabItem = { value: string; label: string; content: ReactNode };

export type TabsProps = {
  items: TabItem[];
  /** Controlled active value; falls back to the first item. */
  value?: string;
  onValueChange?: (value: string) => void;
  className?: string;
};

/** Tab bar with an animated active indicator + content panel. */
export function Tabs({ items, value, onValueChange, className }: TabsProps) {
  const [internal, setInternal] = useState(items[0]?.value);
  const active = value ?? internal;
  const activeIndex = Math.max(0, items.findIndex((i) => i.value === active));
  const select = (v: string) => {
    setInternal(v);
    onValueChange?.(v);
  };
  const pct = items.length ? 100 / items.length : 100;

  return (
    <View className={className}>
      <View className="relative flex-row rounded-lg bg-muted p-1">
        {/* Sliding indicator */}
        <MotiView
          className="absolute bottom-1 top-1 rounded-md bg-card"
          animate={{ left: `${activeIndex * pct}%` }}
          transition={{ type: 'spring', damping: 18, stiffness: 220 }}
          style={{ width: `${pct}%` }}
          pointerEvents="none"
        />
        {items.map((item) => {
          const isActive = item.value === active;
          return (
            <Pressable
              key={item.value}
              onPress={() => select(item.value)}
              accessibilityRole="tab"
              accessibilityState={{ selected: isActive }}
              className="flex-1 items-center justify-center px-3 py-2"
            >
              <Text
                className={cn(
                  'font-body-medium text-sm',
                  isActive ? 'text-foreground' : 'text-muted-foreground',
                )}
              >
                {item.label}
              </Text>
            </Pressable>
          );
        })}
      </View>
      {/* key on `active` so the panel cross-fades when the tab changes */}
      <MotiView
        key={active}
        from={{ opacity: 0, translateX: 8 }}
        animate={{ opacity: 1, translateX: 0 }}
        transition={{ type: 'timing', duration: 200 }}
        className="pt-4"
      >
        {items.find((i) => i.value === active)?.content}
      </MotiView>
    </View>
  );
}
