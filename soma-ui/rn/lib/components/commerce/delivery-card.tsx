import { View } from 'react-native';
import { Text } from '@/lib/components/data-display/text';
import { PriceTag } from './price-tag';
import { StatusPill, type StatusTone } from './status-pill';
import { cn } from '@/lib/utils/cn';

export type Delivery = {
  id: string;
  pickup: string;
  dropoff: string;
  distanceKm: number;
  payout: number;
  status?: { label: string; tone: StatusTone; live?: boolean };
};

export type DeliveryCardProps = {
  delivery: Delivery;
  className?: string;
};

/** A delivery job card: pickup → dropoff route, distance, payout, status. */
export function DeliveryCard({ delivery, className }: DeliveryCardProps) {
  return (
    <View className={cn('gap-3 rounded-xl border border-border bg-card p-4', className)}>
      <View className="flex-row items-center justify-between">
        {delivery.status ? (
          <StatusPill label={delivery.status.label} tone={delivery.status.tone} live={delivery.status.live} />
        ) : (
          <StatusPill label={`${delivery.distanceKm.toFixed(1)} km`} tone="neutral" />
        )}
        <PriceTag amount={delivery.payout} size="md" />
      </View>

      {/* Route */}
      <View className="flex-row gap-3">
        <View className="items-center pt-1">
          <View className="h-2.5 w-2.5 rounded-full border-2 border-primary" />
          <View className="my-0.5 h-6 w-0.5 bg-border" />
          <View className="h-2.5 w-2.5 rounded-full bg-success" />
        </View>
        <View className="flex-1 gap-3">
          <View>
            <Text variant="small" className="text-muted-foreground">Pickup</Text>
            <Text variant="p" numberOfLines={1} className="text-sm font-body-medium">{delivery.pickup}</Text>
          </View>
          <View>
            <Text variant="small" className="text-muted-foreground">Dropoff</Text>
            <Text variant="p" numberOfLines={1} className="text-sm font-body-medium">{delivery.dropoff}</Text>
          </View>
        </View>
      </View>
    </View>
  );
}
