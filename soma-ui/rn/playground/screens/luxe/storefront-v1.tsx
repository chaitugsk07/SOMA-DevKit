import { useState } from 'react';
import { View, Image, Pressable, StyleSheet } from 'react-native';
import { MotiView, AnimatePresence } from 'moti';
import { Text } from '@/lib/components';
import { LUXE_PRODUCTS } from './data';

/** V1 — Full-bleed cinematic carousel. Image IS the screen; type overlaid. */
export function LuxeStorefrontV1() {
  const [i, setI] = useState(0);
  const p = LUXE_PRODUCTS[i];
  const next = () => setI((v) => (v + 1) % LUXE_PRODUCTS.length);
  const prev = () => setI((v) => (v - 1 + LUXE_PRODUCTS.length) % LUXE_PRODUCTS.length);

  return (
    <View className="flex-1 bg-black">
      {/* Full-bleed image */}
      <AnimatePresence>
        <MotiView
          key={p.id}
          from={{ opacity: 0, scale: 1.08 }}
          animate={{ opacity: 1, scale: 1 }}
          transition={{ type: 'timing', duration: 700 }}
          style={StyleSheet.absoluteFill}
        >
          <Image source={{ uri: p.image }} style={StyleSheet.absoluteFill} className="h-full w-full" resizeMode="cover" />
          {/* gradient scrims — layered translucent bands so type stays legible
              without blacking out the product */}
          <View className="absolute inset-x-0 top-0 h-24 bg-black/25" />
          <View className="absolute inset-x-0 bottom-0 h-56 bg-black/35" />
          <View className="absolute inset-x-0 bottom-0 h-28 bg-black/45" />
        </MotiView>
      </AnimatePresence>

      {/* Top bar */}
      <View className="mt-4 items-center pt-8">
        <View className="w-full max-w-4xl flex-row items-center justify-between px-6">
          <Text className="font-heading-bold text-lg tracking-[4px] text-white">SOMA</Text>
          <Text className="text-xs tracking-widest text-white/70">{String(i + 1).padStart(2, '0')} / {String(LUXE_PRODUCTS.length).padStart(2, '0')}</Text>
        </View>
      </View>

      {/* Tap zones for prev/next */}
      <Pressable className="absolute inset-y-0 left-0 w-1/3" onPress={prev} />
      <Pressable className="absolute inset-y-0 right-0 w-1/3" onPress={next} />

      {/* Overlaid editorial type */}
      <View className="flex-1 items-center justify-end pb-10">
        <View className="w-full max-w-4xl px-6">
          <MotiView key={`t${p.id}`} from={{ opacity: 0, translateY: 20 }} animate={{ opacity: 1, translateY: 0 }} transition={{ type: 'timing', duration: 500, delay: 150 }}>
            {p.tag && (
              <View className="mb-3 self-start border border-white/40 px-2 py-0.5">
                <Text className="text-[10px] tracking-[3px] text-white">{p.tag}</Text>
              </View>
            )}
            <Text className="text-xs uppercase tracking-[3px] text-white/70">{p.subtitle}</Text>
            <Text className="font-heading-bold text-6xl leading-[0.95] text-white">{p.name}</Text>
            <View className="mt-5 flex-row items-center justify-between">
              <Text className="font-heading-semibold text-2xl text-white">${p.price.toLocaleString()}</Text>
              <Pressable onPress={next} className="flex-row items-center gap-2 border-b border-white pb-1 active:opacity-60">
                <Text className="text-sm tracking-widest text-white">EXPLORE</Text>
                <Text className="text-white">→</Text>
              </Pressable>
            </View>
          </MotiView>

          {/* Progress dots */}
          <View className="mt-8 flex-row gap-1.5">
            {LUXE_PRODUCTS.map((_, idx) => (
              <View key={idx} className={'h-0.5 flex-1 ' + (idx === i ? 'bg-white' : 'bg-white/30')} />
            ))}
          </View>
        </View>
      </View>
    </View>
  );
}
