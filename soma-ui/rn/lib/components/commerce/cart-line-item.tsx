import { View, Image, Pressable } from 'react-native';
import { Text } from '@/lib/components/data-display/text';
import { PriceTag } from './price-tag';
import { QuantityStepper } from './quantity-stepper';
import type { Product } from './product-card';

export type CartLineItemProps = {
  product: Product;
  quantity: number;
  onQuantityChange: (q: number) => void;
  onRemove?: () => void;
};

/** A single cart row: thumb, name, price, quantity stepper, remove. */
export function CartLineItem({ product, quantity, onQuantityChange, onRemove }: CartLineItemProps) {
  return (
    <View className="flex-row gap-3 py-3">
      <View className="h-16 w-16 items-center justify-center overflow-hidden rounded-lg bg-muted">
        {product.image ? (
          <Image source={{ uri: product.image }} className="h-full w-full" resizeMode="cover" />
        ) : (
          <Text className="font-heading-bold text-lg text-muted-foreground">{product.name.charAt(0)}</Text>
        )}
      </View>
      <View className="flex-1 justify-between">
        <View className="flex-row items-start justify-between">
          <Text variant="p" numberOfLines={1} className="flex-1 font-body-medium">{product.name}</Text>
          {onRemove && (
            <Pressable onPress={onRemove} accessibilityLabel="Remove item" className="ml-2 active:opacity-60">
              <Text className="text-sm text-muted-foreground">✕</Text>
            </Pressable>
          )}
        </View>
        <View className="flex-row items-center justify-between">
          <QuantityStepper value={quantity} onChange={onQuantityChange} />
          <PriceTag amount={product.price * quantity} size="sm" />
        </View>
      </View>
    </View>
  );
}
