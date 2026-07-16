import { View, Pressable, Image } from 'react-native';
import { Text } from '@/lib/components/data-display/text';
import { ProductAccentDot } from './atoms';
import { type ProductAccent, productAccents } from '@/lib/theme/tokens';
import { cn } from '@/lib/utils/cn';

const rupee = (n: number) => `₹${n.toLocaleString('en-IN')}`;

// ─── DailyCostBadge ──────────────────────────────────────────────────────────
/** "₹70/day" framing under a monthly price. */
export function DailyCostBadge({ perDay, className }: { perDay: number; className?: string }) {
  return (
    <View className={cn('self-start rounded-full bg-muted px-2.5 py-1', className)}>
      <Text className="font-body-medium text-xs text-card-foreground">{rupee(perDay)}/day</Text>
    </View>
  );
}

// ─── AnchoredPriceDisplay ────────────────────────────────────────────────────
/** Retail (struck) vs plan price with savings. */
export function AnchoredPriceDisplay({ retail, price, className }: { retail: number; price: number; className?: string }) {
  const saved = retail - price;
  const pct = Math.round((saved / retail) * 100);
  return (
    <View className={cn('flex-row items-baseline gap-2', className)}>
      <Text className="font-heading-bold text-2xl text-card-foreground">{rupee(price)}</Text>
      <Text className="font-body text-sm text-muted-foreground line-through">{rupee(retail)}</Text>
      {saved > 0 && <Text className="font-body-medium text-xs text-card-foreground">Save {pct}%</Text>}
    </View>
  );
}

// ─── PlanCard ────────────────────────────────────────────────────────────────
export type Plan = {
  name: string;
  monthly: number;
  perDay: number;
  bottlesPerWeek: number;
  deliveryDays: string;
  featured?: boolean;
};

export function PlanCard({ plan, selected, onPress, className }: { plan: Plan; selected?: boolean; onPress?: () => void; className?: string }) {
  return (
    <Pressable onPress={onPress} accessibilityRole="button" accessibilityState={{ selected }}
      className={cn('gap-3 rounded-2xl border bg-card p-5', selected ? 'border-primary' : 'border-border', className)}>
      <View className="flex-row items-center justify-between">
        <Text className="font-heading-semibold text-lg text-card-foreground">{plan.name}</Text>
        {plan.featured && (
          <View className="rounded-full bg-primary px-2 py-0.5">
            <Text className="font-body-medium text-[10px] uppercase tracking-wide text-primary-foreground">Popular</Text>
          </View>
        )}
      </View>
      <View className="flex-row items-baseline gap-1.5">
        <Text className="font-heading-bold text-3xl text-card-foreground">{rupee(plan.monthly)}</Text>
        <Text className="font-body text-sm text-muted-foreground">/ month</Text>
      </View>
      <DailyCostBadge perDay={plan.perDay} />
      <View className="gap-1 pt-1">
        <Text className="font-body text-sm text-muted-foreground">{plan.bottlesPerWeek} bottles / week</Text>
        <Text className="font-body text-sm text-muted-foreground">Delivered {plan.deliveryDays}</Text>
      </View>
    </Pressable>
  );
}

// ─── JuiceCard ───────────────────────────────────────────────────────────────
export type Juice = {
  name: string;
  accent: ProductAccent;
  volumeMl: number;
  price?: number;
  image?: string;
};

// Stylized bottle silhouette — shown when no product image is available.
// Fills the card's aspect area with a soft accent wash; neck cap + body + label
// initial make it read as a deliberate illustration rather than a plain swatch.
function BottlePlaceholder({ juice }: { juice: Pick<Juice, 'accent' | 'name'> }) {
  const hex = productAccents[juice.accent];
  const letter = juice.name.charAt(0).toUpperCase();
  return (
    <View style={{ backgroundColor: hex + '14' }} className="h-full w-full items-center justify-center">
      <View className="items-center">
        {/* Neck cap */}
        <View style={{ width: 28, height: 16, borderRadius: 6, backgroundColor: hex + '55' }} />
        {/* Bottle body */}
        <View
          style={{ width: 68, height: 116, backgroundColor: hex + '30' }}
          className="rounded-t-lg rounded-b-xl overflow-hidden items-center justify-center"
        >
          {/* Label band across the body's middle */}
          <View className="bg-card w-full py-1 items-center">
            <Text style={{ color: hex }} className="font-heading-bold text-lg">{letter}</Text>
          </View>
        </View>
      </View>
    </View>
  );
}

export function JuiceCard({
  juice,
  onPress,
  className,
  accessibilityLabel,
}: {
  juice: Juice;
  onPress?: () => void;
  className?: string;
  accessibilityLabel?: string;
}) {
  return (
    <Pressable
      onPress={onPress}
      accessibilityRole="button"
      accessibilityLabel={accessibilityLabel ?? juice.name}
      className={cn('overflow-hidden rounded-xl bg-card', className)}
    >
      <View className="aspect-[4/5] w-full items-center justify-center bg-muted">
        {juice.image ? (
          <Image source={{ uri: juice.image }} className="h-full w-full" resizeMode="cover" />
        ) : (
          <BottlePlaceholder juice={juice} />
        )}
        <View className="absolute right-2 top-2 rounded-full bg-card px-2 py-0.5">
          <Text className="font-body text-[10px] text-card-foreground">{juice.volumeMl}ml</Text>
        </View>
      </View>
      <View className="gap-1 p-3">
        <View className="flex-row items-center gap-2" style={{ minHeight: 40 }}>
          <ProductAccentDot accent={juice.accent} size={8} />
          <Text className="flex-1 font-body-medium text-sm text-card-foreground" numberOfLines={2}>{juice.name}</Text>
        </View>
        {juice.price != null && (
          <Text className="font-heading-semibold text-base text-card-foreground">{rupee(juice.price)}</Text>
        )}
      </View>
    </Pressable>
  );
}
