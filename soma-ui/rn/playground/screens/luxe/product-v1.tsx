import { View, ScrollView, Image, Pressable } from 'react-native';
import { MotiView } from 'moti';
import { Text } from '@/lib/components';
import { LUXE_PRODUCTS } from './data';

const p = LUXE_PRODUCTS[0]; // Orbit headphones

/** V1 — Immersive. Full-bleed hero, detail card overlaps up onto the image. */
export function LuxeProductV1() {
  return (
    <View className="flex-1 items-center bg-background">
      <View className="w-full max-w-4xl flex-1">
        <ScrollView contentContainerClassName="pb-28" showsVerticalScrollIndicator={false}>
          {/* Hero */}
          <MotiView from={{ opacity: 0, scale: 1.06 }} animate={{ opacity: 1, scale: 1 }} transition={{ type: 'timing', duration: 700 }}>
            <View className="h-[440px] w-full bg-muted">
              <Image source={{ uri: p.image }} className="h-full w-full" resizeMode="cover" />
              <View className="absolute inset-x-0 top-0 h-28 bg-black/25" />
            </View>
          </MotiView>

          {/* Back + wishlist over image */}
          <View className="absolute inset-x-0 top-0 flex-row justify-between px-6 pt-10">
            <Text className="text-xl text-white">‹</Text>
            <Text className="text-lg text-white">♡</Text>
          </View>

          {/* Overlapping detail sheet */}
          <MotiView
            from={{ opacity: 0, translateY: 30 }}
            animate={{ opacity: 1, translateY: 0 }}
            transition={{ type: 'timing', duration: 500, delay: 250 }}
            className="-mt-8 gap-5 rounded-t-3xl bg-background px-6 pt-8"
          >
            <View>
              <Text className="text-xs uppercase tracking-[3px] text-muted-foreground">{p.subtitle}</Text>
              <Text className="font-heading-bold text-5xl leading-[0.95] text-foreground">{p.name}</Text>
            </View>

            {/* Color swatches */}
            <View className="flex-row gap-2">
              {['bg-foreground', 'bg-primary', 'bg-muted-foreground'].map((c, idx) => (
                <View key={c} className={'h-7 w-7 rounded-full border-2 ' + (idx === 0 ? 'border-primary' : 'border-transparent') + ' ' + c} />
              ))}
            </View>

            <Text className="leading-6 text-muted-foreground">
              Reference-grade wireless headphones. Adaptive noise cancellation, 40 hours of playback,
              machined aluminium — engineered for stillness.
            </Text>

            <View className="flex-row gap-6 border-t border-border pt-5">
              <Spec label="Battery" value="40h" />
              <Spec label="Weight" value="248g" />
              <Spec label="Driver" value="40mm" />
            </View>
          </MotiView>
        </ScrollView>

        {/* Sticky luxe CTA */}
        <View className="absolute inset-x-0 bottom-0 flex-row items-center gap-4 border-t border-border bg-background px-6 py-4">
          <View>
            <Text className="text-[10px] uppercase tracking-widest text-muted-foreground">Price</Text>
            <Text className="font-heading-bold text-2xl text-foreground">${p.price}</Text>
          </View>
          <Pressable className="flex-1 items-center justify-center rounded-full bg-foreground py-4 active:opacity-80">
            <Text className="text-sm tracking-[2px] text-background">ADD TO BAG</Text>
          </Pressable>
        </View>
      </View>
    </View>
  );
}

function Spec({ label, value }: { label: string; value: string }) {
  return (
    <View>
      <Text className="text-[10px] uppercase tracking-widest text-muted-foreground">{label}</Text>
      <Text className="font-heading-semibold text-lg text-foreground">{value}</Text>
    </View>
  );
}
