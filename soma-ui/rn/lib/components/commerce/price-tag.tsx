import { View } from 'react-native';
import { Text } from '@/lib/components/data-display/text';
import { cn } from '@/lib/utils/cn';

export type PriceTagProps = {
  /** Price in major units (e.g. 49.99). */
  amount: number;
  /** Original price for a strikethrough sale display. */
  was?: number;
  currency?: string;
  size?: 'sm' | 'md' | 'lg';
  className?: string;
  priceClassName?: string;
  wasClassName?: string;
};

const fmt = (n: number, currency: string) =>
  `${currency}${n.toLocaleString('en-IN', {
    minimumFractionDigits: Number.isInteger(n) ? 0 : 2,
    maximumFractionDigits: 2,
  })}`;

const sizeMap = {
  sm: { price: 'text-sm', was: 'text-xs' },
  md: { price: 'text-lg', was: 'text-sm' },
  lg: { price: 'text-3xl', was: 'text-base' },
} as const;

/** Price display with optional strikethrough original. */
export function PriceTag({
  amount,
  was,
  currency = '$',
  size = 'md',
  className,
  priceClassName,
  wasClassName,
}: PriceTagProps) {
  const s = sizeMap[size];
  return (
    <View className={cn('flex-row items-baseline gap-2', className)}>
      <Text className={cn('font-body-semibold text-foreground', s.price, priceClassName)}>
        {fmt(amount, currency)}
      </Text>
      {was != null && was > amount && (
        <Text className={cn('font-body text-muted-foreground line-through', s.was, wasClassName)}>
          {fmt(was, currency)}
        </Text>
      )}
    </View>
  );
}
