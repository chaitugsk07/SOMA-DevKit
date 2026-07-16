import { View, ScrollView, Image, Pressable } from 'react-native';
import { MotiView } from 'moti';
import { Text } from '@/lib/components';
import { LUXE_PRODUCTS } from './data';

const BAG = [LUXE_PRODUCTS[0], LUXE_PRODUCTS[4]];

/** V3 — The Bag. Editorial cart review with large imagery, dramatic total. */
export function LuxeCheckoutV3() {
  const total = BAG.reduce((n, p) => n + p.price, 0);
  return (
    <View className="flex-1 items-center bg-background">
      <View className="w-full max-w-3xl flex-1">
        <ScrollView contentContainerClassName="pb-40" showsVerticalScrollIndicator={false}>
          <View className="px-6 pb-4 pt-14">
            <View className="flex-row items-center justify-between">
              <Text className="text-xl text-foreground">‹</Text>
              <Text className="text-xs uppercase tracking-[3px] text-muted-foreground">{BAG.length} items</Text>
            </View>
            <Text className="mt-4 font-heading-bold text-6xl leading-[0.92] text-foreground">Your{'\n'}Bag</Text>
          </View>

          {BAG.map((p, i) => (
            <MotiView
              key={p.id}
              from={{ opacity: 0, translateX: 20 }}
              animate={{ opacity: 1, translateX: 0 }}
              transition={{ type: 'timing', duration: 450, delay: 150 + i * 120 }}
            >
              <View className="flex-row gap-4 border-t border-border px-6 py-5">
                <View className="h-28 w-24 overflow-hidden rounded-md bg-muted">
                  <Image source={{ uri: p.image }} className="h-full w-full" resizeMode="cover" />
                </View>
                <View className="flex-1 justify-between py-1">
                  <View>
                    <Text className="font-heading-bold text-2xl leading-tight text-foreground">{p.name}</Text>
                    <Text className="text-[11px] uppercase tracking-[2px] text-muted-foreground">{p.subtitle}</Text>
                  </View>
                  <View className="flex-row items-center justify-between">
                    <View className="flex-row items-center gap-3">
                      <Text className="text-lg text-muted-foreground">−</Text>
                      <Text className="font-body-medium text-sm text-foreground">1</Text>
                      <Text className="text-lg text-muted-foreground">+</Text>
                    </View>
                    <Text className="font-heading-semibold text-lg text-foreground">${p.price.toLocaleString()}</Text>
                  </View>
                </View>
              </View>
            </MotiView>
          ))}

          <View className="mt-6 px-6">
            <Pressable className="flex-row items-center gap-2 active:opacity-60">
              <Text className="text-sm text-muted-foreground">＋ Add a promo code</Text>
            </Pressable>
          </View>
        </ScrollView>

        {/* Dramatic total bar */}
        <View className="absolute inset-x-0 bottom-0 gap-3 border-t border-border bg-background px-6 pb-6 pt-4">
          <View className="flex-row items-end justify-between">
            <Text className="text-xs uppercase tracking-[3px] text-muted-foreground">Total</Text>
            <Text className="font-heading-bold text-4xl text-foreground">${total.toLocaleString()}</Text>
          </View>
          <Pressable className="items-center justify-center rounded-full bg-foreground py-4 active:opacity-80">
            <Text className="text-sm tracking-[2px] text-background">CHECKOUT</Text>
          </Pressable>
        </View>
      </View>
    </View>
  );
}
