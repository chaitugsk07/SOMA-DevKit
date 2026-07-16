import { useState, type ReactNode } from 'react';
import { Pressable, View } from 'react-native';
import { AnimatePresence, MotiView } from 'moti';
import { Text } from '@/lib/components/data-display/text';

export type TooltipProps = {
  /** The trigger element. */
  children: ReactNode;
  label: string;
};

/** Tap-to-toggle tooltip bubble above the trigger (touch has no hover). */
export function Tooltip({ children, label }: TooltipProps) {
  const [open, setOpen] = useState(false);
  return (
    <View className="items-center">
      <AnimatePresence>
        {open && (
          <MotiView
            from={{ opacity: 0, translateY: 4 }}
            animate={{ opacity: 1, translateY: 0 }}
            exit={{ opacity: 0, translateY: 4 }}
            transition={{ type: 'timing', duration: 150 }}
            className="absolute bottom-full mb-2 rounded-md bg-foreground px-2 py-1"
          >
            <Text className="font-body text-xs text-background">{label}</Text>
          </MotiView>
        )}
      </AnimatePresence>
      <Pressable
        onPress={() => setOpen((o) => !o)}
        accessibilityLabel={label}
        accessibilityRole="button"
      >
        {children}
      </Pressable>
    </View>
  );
}
