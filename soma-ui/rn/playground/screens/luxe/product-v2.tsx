import { View, Image, Pressable } from 'react-native';
import { MotiView } from 'moti';
import { Text } from '@/lib/components';
import { LUXE_PRODUCTS } from './data';

const p = LUXE_PRODUCTS[1]; // Meridian watch

/** V2 — Split editorial. Type block over image block, hard divide. */
export function LuxeProductV2() {
  return (
    <View className="flex-1 items-center bg-background">
      <View className="w-full max-w-3xl flex-1">
        {/* Top — typographic block */}
        <View className="justify-center px-6 pt-14" style={{ flex: 1 }}>
          <View className="flex-row items-center justify-between">
            <Text className="text-xl text-foreground">‹</Text>
            <Text className="text-xs uppercase tracking-[3px] text-muted-foreground">Timepieces</Text>
            <Text className="text-lg text-foreground">♡</Text>
          </View>
          <MotiView from={{ opacity: 0, translateY: 16 }} animate={{ opacity: 1, translateY: 0 }} transition={{ type: 'timing', duration: 500, delay: 150 }} className="mt-auto pb-6">
            <Text className="font-heading-bold text-7xl leading-[0.9] text-foreground">{p.name}</Text>
            <Text className="mt-1 text-sm uppercase tracking-[2px] text-muted-foreground">{p.subtitle}</Text>
            <View className="mt-4 flex-row items-end gap-3">
              <Text className="font-heading-bold text-3xl text-foreground">${p.price.toLocaleString()}</Text>
              <Text className="pb-1 text-xs text-muted-foreground">or 4 × ${Math.round(p.price / 4)}</Text>
            </View>
          </MotiView>
        </View>

        {/* Bottom — full-bleed image block */}
        <MotiView from={{ opacity: 0 }} animate={{ opacity: 1 }} transition={{ type: 'timing', duration: 700 }} style={{ flex: 1 }}>
          <Image source={{ uri: p.image }} className="h-full w-full" resizeMode="cover" />
          {/* Floating CTA on the image */}
          <View className="absolute inset-x-0 bottom-0 p-6">
            <Pressable className="flex-row items-center justify-between rounded-full bg-background/95 px-6 py-4 active:opacity-80">
              <Text className="text-sm tracking-[2px] text-foreground">ADD TO BAG</Text>
              <View className="h-8 w-8 items-center justify-center rounded-full bg-primary">
                <Text className="text-background">→</Text>
              </View>
            </Pressable>
          </View>
        </MotiView>
      </View>
    </View>
  );
}
