import { View, Image, Pressable } from 'react-native';
import { MotiView } from 'moti';
import { Text } from '@/lib/components/data-display/text';
import { Badge } from '@/lib/components/data-display/badge';
import { PriceTag } from './price-tag';
import { Rating } from './rating';

export type Product = {
  id: string;
  name: string;
  price: number;
  was?: number;
  image?: string;
  rating?: number;
  ratingCount?: number;
  badge?: string;
};

export type ProductCardProps = {
  product: Product;
  onPress?: () => void;
  className?: string;
};

/** Storefront product tile — image, name, rating, price, with a press-scale. */
export function ProductCard({ product, onPress, className }: ProductCardProps) {
  return (
    <Pressable onPress={onPress} accessibilityRole="button" className={className}>
      {({ pressed }) => (
        <MotiView
          animate={{ scale: pressed ? 0.97 : 1 }}
          transition={{ type: 'spring', damping: 18, stiffness: 260 }}
          className="overflow-hidden rounded-xl border border-border bg-card"
        >
          <View className="aspect-square w-full items-center justify-center bg-muted">
            {product.image ? (
              <Image source={{ uri: product.image }} className="h-full w-full" resizeMode="cover" />
            ) : (
              <Text className="font-heading-bold text-2xl text-muted-foreground">
                {product.name.charAt(0)}
              </Text>
            )}
            {product.badge && (
              <View className="absolute left-2 top-2">
                <Badge variant="destructive" label={product.badge} />
              </View>
            )}
          </View>
          <View className="gap-1.5 p-3">
            <Text variant="p" numberOfLines={1} className="font-body-medium">{product.name}</Text>
            {product.rating != null && <Rating value={product.rating} count={product.ratingCount} />}
            <PriceTag amount={product.price} was={product.was} size="md" />
          </View>
        </MotiView>
      )}
    </Pressable>
  );
}
