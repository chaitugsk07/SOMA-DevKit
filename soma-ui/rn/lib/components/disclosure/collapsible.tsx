import { useState, type ReactNode } from 'react';
import { Pressable, View, LayoutAnimation, Platform, UIManager } from 'react-native';
import { Text } from '@/lib/components/data-display/text';
import { cn } from '@/lib/utils/cn';

// ponytail: LayoutAnimation gives smooth open/close height for free (no measuring).
// Ceiling: it animates the whole next layout pass, not just this view. Fine for
// disclosure rows; swap to a measured-height Moti animation if that ever bites.
if (Platform.OS === 'android' && UIManager.setLayoutAnimationEnabledExperimental) {
  UIManager.setLayoutAnimationEnabledExperimental(true);
}

export type CollapsibleProps = {
  title: string;
  children: ReactNode;
  defaultOpen?: boolean;
  className?: string;
};

/** Single expand/collapse section with animated height. */
export function Collapsible({ title, children, defaultOpen = false, className }: CollapsibleProps) {
  const [open, setOpen] = useState(defaultOpen);
  const toggle = () => {
    LayoutAnimation.configureNext(LayoutAnimation.Presets.easeInEaseOut);
    setOpen((o) => !o);
  };
  return (
    <View className={cn('overflow-hidden rounded-lg border border-border', className)}>
      <Pressable
        onPress={toggle}
        accessibilityRole="button"
        accessibilityState={{ expanded: open }}
        className="flex-row items-center justify-between bg-card px-4 py-3 active:bg-accent"
      >
        <Text className="font-body-medium text-sm text-foreground">{title}</Text>
        <Text className="text-muted-foreground">{open ? '▾' : '▸'}</Text>
      </Pressable>
      {open && <View className="bg-card px-4 pb-4">{children}</View>}
    </View>
  );
}
