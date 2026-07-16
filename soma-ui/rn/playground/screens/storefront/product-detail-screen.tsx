import { useState } from 'react';
import { View, ScrollView, Image } from 'react-native';
import { AnimatePresence, MotiView } from 'moti';
import {
  Text,
  Button,
  Badge,
  Separator,
  PriceTag,
  Rating,
  QuantityStepper,
  FadeIn,
  Reveal,
} from '@/lib/components';
import { PRODUCTS } from './data';

const product = PRODUCTS[0]; // "Aero Runner"

/** Product detail: hero image, info, quantity, animated add-to-cart confirmation. */
export function ProductDetailScreen() {
  const [qty, setQty] = useState(1);
  const [added, setAdded] = useState(false);

  const addToCart = () => {
    setAdded(true);
    setTimeout(() => setAdded(false), 1600);
  };

  return (
    <View className="flex-1 bg-background">
      <ScrollView contentContainerClassName="pb-28 items-center">
        <View className="w-full max-w-3xl">
          {/* Hero image */}
          <FadeIn>
            <View className="aspect-square w-full bg-muted">
              <Image source={{ uri: product.image }} className="h-full w-full" resizeMode="cover" />
            </View>
          </FadeIn>

          <Reveal direction="up" className="gap-4 p-5">
            <View className="flex-row items-start justify-between">
              <View className="flex-1 gap-1">
                <Text variant="h1">{product.name}</Text>
                <Rating value={product.rating!} count={product.ratingCount} size="md" />
              </View>
              {product.badge && <Badge variant="destructive" label={product.badge} />}
            </View>

            <PriceTag amount={product.price} was={product.was} size="lg" />

            <Separator />

            <Text variant="p" className="leading-6 text-muted-foreground">
              Lightweight performance runner with a breathable knit upper and a responsive foam sole.
              Built for daily miles, finished in slate.
            </Text>

            <View className="flex-row items-center justify-between">
              <Text variant="p" className="font-body-medium">Quantity</Text>
              <QuantityStepper value={qty} onChange={setQty} />
            </View>
          </Reveal>
        </View>
      </ScrollView>

      {/* Sticky add-to-cart bar */}
      <View className="absolute inset-x-0 bottom-0 flex-row items-center gap-3 border-t border-border bg-card p-4">
        <View>
          <Text variant="small" className="text-muted-foreground">Total</Text>
          <PriceTag amount={product.price * qty} size="md" />
        </View>
        <Button className="flex-1" size="lg" label={added ? 'Added ✓' : 'Add to cart'} onPress={addToCart} />
      </View>

      {/* Add confirmation toast-ish pill */}
      <AnimatePresence>
        {added && (
          <MotiView
            from={{ opacity: 0, translateY: -12 }}
            animate={{ opacity: 1, translateY: 0 }}
            exit={{ opacity: 0, translateY: -12 }}
            transition={{ type: 'timing', duration: 200 }}
            className="absolute inset-x-0 top-4 items-center"
            pointerEvents="none"
          >
            <View className="rounded-full bg-success px-4 py-2">
              <Text className="font-body-medium text-sm text-success-foreground">
                Added {qty} × {product.name}
              </Text>
            </View>
          </MotiView>
        )}
      </AnimatePresence>
    </View>
  );
}
