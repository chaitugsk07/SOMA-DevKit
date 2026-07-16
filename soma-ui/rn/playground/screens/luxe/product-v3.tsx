import { View, ScrollView, Image, Pressable } from 'react-native';
import { MotiView } from 'moti';
import { Text } from '@/lib/components';
import { LUXE_PRODUCTS } from './data';

const p = LUXE_PRODUCTS[2]; // Atelier weekender

/** V3 — Gallery minimal. Extreme whitespace, centred object, hairline specs (Aesop). */
export function LuxeProductV3() {
  return (
    <View className="flex-1 items-center bg-background">
      <View className="w-full max-w-3xl flex-1">
        <ScrollView contentContainerClassName="pb-28" showsVerticalScrollIndicator={false}>
          <View className="flex-row items-center justify-between px-6 pt-12">
            <Text className="text-xl text-foreground">‹</Text>
            <Text className="font-heading-bold text-sm tracking-[3px] text-foreground">SOMA</Text>
            <Text className="text-lg text-foreground">♡</Text>
          </View>

          {/* Centred product in generous space */}
          <MotiView from={{ opacity: 0, scale: 0.94 }} animate={{ opacity: 1, scale: 1 }} transition={{ type: 'timing', duration: 600 }} className="items-center px-10 py-12">
            <View className="aspect-square w-full overflow-hidden rounded-md bg-muted">
              <Image source={{ uri: p.image }} className="h-full w-full" resizeMode="cover" />
            </View>
          </MotiView>

          {/* Understated info */}
          <MotiView from={{ opacity: 0, translateY: 12 }} animate={{ opacity: 1, translateY: 0 }} transition={{ type: 'timing', duration: 500, delay: 250 }} className="items-center gap-2 px-6">
            <Text className="text-[11px] uppercase tracking-[4px] text-muted-foreground">{p.subtitle}</Text>
            <Text className="font-heading-bold text-4xl text-foreground">{p.name}</Text>
            <Text className="font-body text-base text-muted-foreground">${p.price.toLocaleString()}</Text>
          </MotiView>

          {/* Hairline spec table */}
          <View className="mt-10 px-6">
            {[['Material', 'Full-grain leather'], ['Capacity', '38 litres'], ['Made in', 'Florence, IT']].map(([k, v], i) => (
              <View key={k} className={'flex-row justify-between py-4 ' + (i > 0 ? 'border-t border-border' : 'border-y border-border')}>
                <Text className="text-xs uppercase tracking-widest text-muted-foreground">{k}</Text>
                <Text className="font-body-medium text-sm text-foreground">{v}</Text>
              </View>
            ))}
          </View>
        </ScrollView>

        {/* Minimal outlined CTA */}
        <View className="absolute inset-x-0 bottom-0 bg-background px-6 py-4">
          <Pressable className="items-center justify-center rounded-none border border-foreground py-4 active:bg-accent">
            <Text className="text-sm tracking-[3px] text-foreground">ADD TO BAG — ${p.price.toLocaleString()}</Text>
          </Pressable>
        </View>
      </View>
    </View>
  );
}
