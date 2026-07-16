import { type ReactNode } from 'react';
import { View, Pressable } from 'react-native';
import { useSafeAreaInsets } from 'react-native-safe-area-context';
import { MotiView } from 'moti';
import { Text } from '@/lib/components/data-display/text';
import { cn } from '@/lib/utils/cn';

export type TabBarItem = {
  key: string;
  label: string;
  /** Legacy emoji/text icon — kept for back-compat. Prefer the ReactNode `icon` field. */
  icon?: string;
  /** ReactNode icon (e.g. a lucide-react-native icon). Rendered at 22px; color is applied via className. */
  iconNode?: ReactNode;
  badge?: number;
};

export type BottomTabBarProps = {
  items: TabBarItem[];
  active: string;
  onSelect: (key: string) => void;
  className?: string;
};

/** App bottom navigation with an animated active pill + badges. */
export function BottomTabBar({ items, active, onSelect, className }: BottomTabBarProps) {
  const insets = useSafeAreaInsets();

  return (
    <View
      style={{ paddingBottom: insets.bottom }}
      className={cn('border-t border-border bg-card', className)}
    >
      <View className="flex-row">
        {items.map((item) => {
          const isActive = item.key === active;
          return (
            <Pressable
              key={item.key}
              onPress={() => onSelect(item.key)}
              accessibilityRole="tab"
              accessibilityLabel={item.label}
              accessibilityState={{ selected: isActive }}
              className="flex-1 items-center justify-center py-2.5"
              style={{ minHeight: 44 }}
            >
              {/* Icon area */}
              <View className="relative items-center justify-center">
                {item.iconNode != null ? (
                  <View
                    className={cn(
                      'items-center justify-center rounded-full px-3 py-0.5',
                      isActive ? 'bg-accent' : 'bg-transparent',
                    )}
                  >
                    {/* Clone the icon node with color-appropriate size; callers supply the icon element */}
                    <View style={{ width: 22, height: 22, alignItems: 'center', justifyContent: 'center' }}>
                      {item.iconNode}
                    </View>
                  </View>
                ) : (
                  // Legacy string icon path
                  <Text
                    className={cn('text-xl', isActive ? 'text-accent-foreground' : 'text-muted-foreground')}
                  >
                    {item.icon}
                  </Text>
                )}

                {/* Badge */}
                {item.badge != null && item.badge > 0 && (
                  <View className="absolute -right-1 -top-1 h-4 min-w-4 items-center justify-center rounded-full bg-destructive px-1">
                    <Text className="font-body-semibold text-[10px] text-destructive-foreground">
                      {item.badge}
                    </Text>
                  </View>
                )}
              </View>

              {/* Label */}
              <Text
                className={cn(
                  'mt-0.5 font-body-medium text-xs',
                  isActive ? 'text-accent-foreground' : 'text-muted-foreground',
                )}
              >
                {item.label}
              </Text>

              {/* Active indicator dot */}
              {isActive && (
                <MotiView
                  from={{ opacity: 0, scale: 0.5 }}
                  animate={{ opacity: 1, scale: 1 }}
                  transition={{ type: 'spring', damping: 16, stiffness: 260 }}
                  className="mt-0.5 h-1 w-1 rounded-full bg-accent-foreground"
                />
              )}
            </Pressable>
          );
        })}
      </View>
    </View>
  );
}
