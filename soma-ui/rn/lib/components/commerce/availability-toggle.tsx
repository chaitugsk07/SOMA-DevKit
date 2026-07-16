import { Pressable, View } from 'react-native';
import { MotiView } from 'moti';
import { Text } from '@/lib/components/data-display/text';
import { cn } from '@/lib/utils/cn';

export type AvailabilityToggleProps = {
  online: boolean;
  onToggle: (online: boolean) => void;
  className?: string;
};

/** Large online/offline switch for a driver's availability. */
export function AvailabilityToggle({ online, onToggle, className }: AvailabilityToggleProps) {
  return (
    <Pressable
      onPress={() => onToggle(!online)}
      accessibilityRole="switch"
      accessibilityState={{ checked: online }}
      className={cn(
        'flex-row items-center justify-between rounded-xl border p-4',
        online ? 'border-success bg-success/10' : 'border-border bg-card',
        className,
      )}
    >
      <View className="gap-0.5">
        <Text className={cn('font-heading-semibold text-base', online ? 'text-success' : 'text-foreground')}>
          {online ? "You're online" : "You're offline"}
        </Text>
        <Text variant="muted" className="text-xs">
          {online ? 'Receiving delivery requests' : 'Tap to start receiving jobs'}
        </Text>
      </View>
      <View className={cn('h-7 w-12 justify-center rounded-full px-0.5', online ? 'bg-success' : 'bg-input')}>
        <MotiView
          animate={{ translateX: online ? 20 : 0 }}
          transition={{ type: 'spring', damping: 18, stiffness: 250 }}
          className="h-6 w-6 rounded-full bg-card"
        />
      </View>
    </Pressable>
  );
}
