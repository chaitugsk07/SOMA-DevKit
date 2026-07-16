import { ScrollView, View, Image, Pressable } from 'react-native';
import { MotiView } from 'moti';
import { Text } from '@/lib/components';
import { LUXE_PRODUCTS } from './data';

function Tile({ product, index, tall }: { product: (typeof LUXE_PRODUCTS)[number]; index: number; tall?: boolean }) {
  return (
    <MotiView
      from={{ opacity: 0, translateY: 24 }}
      animate={{ opacity: 1, translateY: 0 }}
      transition={{ type: 'timing', duration: 500, delay: 100 + index * 100 }}
      className="flex-1"
    >
      <Pressable className="active:opacity-80">
        <View className={'overflow-hidden rounded-lg bg-muted ' + (tall ? 'aspect-[3/4]' : 'aspect-square')}>
          <Image source={{ uri: product.image }} className="h-full w-full" resizeMode="cover" />
          <View className="absolute inset-0 justify-end p-3">
            <View className="absolute inset-x-0 bottom-0 h-24 bg-black/45" />
            {product.tag && (
              <View className="absolute right-3 top-3 bg-white px-1.5 py-0.5">
                <Text className="text-[9px] tracking-widest text-black">{product.tag}</Text>
              </View>
            )}
            <Text className="font-heading-bold text-2xl leading-tight text-white">{product.name}</Text>
            <View className="flex-row items-center justify-between">
              <Text className="text-[10px] uppercase tracking-[2px] text-white/80">{product.subtitle}</Text>
              <Text className="font-body-semibold text-sm text-white">${product.price.toLocaleString()}</Text>
            </View>
          </View>
        </View>
      </Pressable>
    </MotiView>
  );
}

/** V3 — Asymmetric gallery grid. One hero tile + staggered pairs, overlaid type. */
export function LuxeStorefrontV3() {
  return (
    <ScrollView className="flex-1 bg-background" contentContainerClassName="items-center px-4 pb-16">
      <View className="w-full max-w-4xl">
        {/* Minimal header */}
        <View className="flex-row items-center justify-between py-5 pt-12">
          <Text className="font-heading-bold text-xl tracking-[3px] text-foreground">SOMA</Text>
          <View className="flex-row gap-4">
            <Text className="text-sm text-muted-foreground">⌕</Text>
            <Text className="text-sm text-muted-foreground">☺</Text>
          </View>
        </View>

        <Text className="mb-4 font-heading-bold text-4xl leading-[0.95] text-foreground">New{'\n'}Arrivals</Text>

        {/* Hero tile */}
        <Tile product={LUXE_PRODUCTS[0]} index={0} tall />

        {/* Staggered pairs */}
        <View className="mt-4 flex-row gap-4">
          <Tile product={LUXE_PRODUCTS[1]} index={1} />
          <Tile product={LUXE_PRODUCTS[3]} index={2} />
        </View>
        <View className="mt-4 flex-row gap-4">
          <Tile product={LUXE_PRODUCTS[2]} index={3} />
          <Tile product={LUXE_PRODUCTS[4]} index={4} />
        </View>
      </View>
    </ScrollView>
  );
}
