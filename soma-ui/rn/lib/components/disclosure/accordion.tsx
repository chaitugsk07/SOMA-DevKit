import { useState, type ReactNode } from 'react';
import { Pressable, View, LayoutAnimation } from 'react-native';
import { Text } from '@/lib/components/data-display/text';
import { cn } from '@/lib/utils/cn';

export type AccordionItem = { value: string; title: string; content: ReactNode };

export type AccordionProps = {
  items: AccordionItem[];
  /** false (default) = single open; true = independent toggles. */
  multiple?: boolean;
  className?: string;
};

/** Stacked disclosure rows. Single-open by default; animated height. */
export function Accordion({ items, multiple = false, className }: AccordionProps) {
  const [open, setOpen] = useState<string[]>([]);
  const toggle = (value: string) => {
    LayoutAnimation.configureNext(LayoutAnimation.Presets.easeInEaseOut);
    setOpen((cur) => {
      const isOpen = cur.includes(value);
      if (multiple) return isOpen ? cur.filter((v) => v !== value) : [...cur, value];
      return isOpen ? [] : [value];
    });
  };
  return (
    <View className={cn('overflow-hidden rounded-lg border border-border', className)}>
      {items.map((item, i) => {
        const isOpen = open.includes(item.value);
        return (
          <View key={item.value} className={cn(i > 0 && 'border-t border-border')}>
            <Pressable
              onPress={() => toggle(item.value)}
              accessibilityRole="button"
              accessibilityState={{ expanded: isOpen }}
              className="flex-row items-center justify-between bg-card px-4 py-3 active:bg-accent"
            >
              <Text className="font-body-medium text-sm text-foreground">{item.title}</Text>
              <Text className="text-muted-foreground">{isOpen ? '▾' : '▸'}</Text>
            </Pressable>
            {isOpen && <View className="bg-card px-4 pb-4">{item.content}</View>}
          </View>
        );
      })}
    </View>
  );
}
