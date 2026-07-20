import { View, Pressable, Image } from 'react-native';
import { PressableScale } from '@/lib/components/motion/pressable-scale';
import { Text } from '@/lib/components/data-display/text';
import { ProductAccentDot } from './atoms';
import { type ProductAccent, productAccents } from '@/lib/theme/tokens';
import { cn } from '@/lib/utils/cn';

const rupee = (n: number) => `₹${n.toLocaleString('en-IN')}`;

// ─── DailyCostBadge ──────────────────────────────────────────────────────────
/** "₹70/day" framing under a monthly price. */
export function DailyCostBadge({ perDay, className }: { perDay: number; className?: string }) {
  return (
    <View className={cn('self-start border-l-2 border-border px-2.5 py-0.5', className)}>
      <Text className="font-body-medium text-xs text-card-foreground">{rupee(perDay)} / day</Text>
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
      <Text className="font-body-semibold text-2xl text-card-foreground">{rupee(price)}</Text>
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
      className={cn('gap-3 rounded-lg border bg-card p-5', selected ? 'border-primary' : 'border-border', className)}>
      <View className="flex-row items-center justify-between">
        <Text className="font-heading-semibold text-lg text-card-foreground">{plan.name}</Text>
        {plan.featured && (
          <View className="rounded-full bg-primary px-2 py-0.5">
            <Text className="font-body-medium text-[10px] uppercase tracking-wide text-primary-foreground">Popular</Text>
          </View>
        )}
      </View>
      <View className="flex-row items-baseline gap-1.5">
        <Text className="font-body-semibold text-3xl text-card-foreground">{rupee(plan.monthly)}</Text>
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
  description?: string;
};

function PressLabelPlaceholder({ juice }: { juice: Pick<Juice, 'accent' | 'name' | 'volumeMl'> }) {
  const hex = productAccents[juice.accent];
  const lightInk = juice.accent === 'beetRuby' ||
    juice.accent === 'berryDeep' ||
    juice.accent === 'earthBrown';
  const ink = lightInk ? '#FAF8F5' : '#2D2D2D';

  return (
    <View style={{ backgroundColor: hex }} className="h-full w-full justify-between p-4">
      <View className="flex-row items-start justify-between">
        <Text style={{ color: ink }} className="font-body-semibold text-[9px] tracking-[2px]">
          SOMA DELIGHTS
        </Text>
        <Text style={{ color: ink }} className="font-body-medium text-[9px] tracking-wider">
          {juice.volumeMl} ML
        </Text>
      </View>
      <View>
        <View className="mb-3 h-px w-10" style={{ backgroundColor: ink }} />
        <Text style={{ color: ink }} className="font-heading-semibold text-2xl leading-7">
          {juice.name}
        </Text>
        <Text style={{ color: ink }} className="mt-3 font-body-medium text-[9px] tracking-[2px]">
          COLD PRESSED · FRESH DAILY
        </Text>
      </View>
      <View className="border-t pt-3" style={{ borderTopColor: `${ink}55` }}>
        <Text style={{ color: ink }} className="font-body text-[9px] tracking-wider">
          KEEP REFRIGERATED
        </Text>
      </View>
    </View>
  );
}

function ProductVisual({ juice }: { juice: Juice }) {
  return (
    <View className="aspect-[4/5] w-full overflow-hidden bg-muted">
      {juice.image ? (
        <Image source={{ uri: juice.image }} className="h-full w-full" resizeMode="cover" />
      ) : (
        <PressLabelPlaceholder juice={juice} />
      )}
      {juice.image ? (
        <View className="absolute bottom-3 left-3 bg-card/90 px-2 py-1">
          <Text className="font-body-medium text-[9px] tracking-wider text-card-foreground">
            {juice.volumeMl} ML
          </Text>
        </View>
      ) : null}
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
  const content = (
    <>
      <ProductVisual juice={juice} />
      <View className="gap-1.5 py-3">
        <View className="flex-row items-start gap-2">
          <ProductAccentDot accent={juice.accent} size={7} className="mt-1.5" />
          <Text className="flex-1 font-body-semibold text-sm text-card-foreground" numberOfLines={2}>
            {juice.name}
          </Text>
        </View>
        {juice.description ? (
          <Text className="font-body text-xs leading-5 text-muted-foreground" numberOfLines={2}>
            {juice.description}
          </Text>
        ) : null}
        {juice.price != null && (
          <Text className="font-body-semibold text-base text-card-foreground">{rupee(juice.price)}</Text>
        )}
      </View>
    </>
  );

  if (!onPress) {
    return (
      <View className={cn('overflow-hidden bg-transparent', className)}>
        {content}
      </View>
    );
  }

  return (
    <PressableScale
      onPress={onPress}
      accessibilityRole="button"
      accessibilityLabel={accessibilityLabel ?? juice.name}
      className={cn('overflow-hidden bg-transparent', className)}
    >
      {content}
    </PressableScale>
  );
}
