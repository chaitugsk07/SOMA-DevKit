import { ScrollView, View, Image, Pressable } from 'react-native';
import { MotiView } from 'moti';
import { Text } from '@/lib/components';
import { LUXE_PRODUCTS } from './data';

/** V2 — Editorial index. Oversized numbered type list, thumbnails as accents. */
export function LuxeStorefrontV2() {
  return (
    <ScrollView className="flex-1 bg-background" contentContainerClassName="items-center pb-16">
      <View className="w-full max-w-3xl">
        {/* Masthead */}
        <View className="px-6 pb-8 pt-14">
          <Text className="text-xs uppercase tracking-[4px] text-muted-foreground">The Collection</Text>
          <Text className="mt-2 font-heading-bold text-5xl leading-[0.95] text-foreground">Fall{'\n'}Objects</Text>
          <View className="mt-6 h-px w-full bg-border" />
        </View>

        {/* Numbered editorial rows */}
        {LUXE_PRODUCTS.map((p, i) => (
          <MotiView
            key={p.id}
            from={{ opacity: 0, translateX: -16 }}
            animate={{ opacity: 1, translateX: 0 }}
            transition={{ type: 'timing', duration: 450, delay: 120 + i * 90 }}
          >
            <Pressable className="active:opacity-70">
              <View className="flex-row items-center gap-4 border-b border-border px-6 py-6">
                <Text className="font-heading-semibold text-sm text-muted-foreground">{String(i + 1).padStart(2, '0')}</Text>
                <View className="flex-1">
                  <Text className="font-heading-bold text-3xl leading-tight text-foreground">{p.name}</Text>
                  <Text className="text-xs uppercase tracking-[2px] text-muted-foreground">{p.subtitle}</Text>
                </View>
                <View className="h-20 w-16 overflow-hidden rounded-sm bg-muted">
                  <Image source={{ uri: p.image }} className="h-full w-full" resizeMode="cover" />
                </View>
                <Text className="font-body-medium text-sm text-foreground">${p.price.toLocaleString()}</Text>
              </View>
            </Pressable>
          </MotiView>
        ))}

        {/* Footer flourish */}
        <View className="items-center px-6 pt-10">
          <Text className="text-xs tracking-[3px] text-muted-foreground">SOMA · EST. 2026</Text>
        </View>
      </View>
    </ScrollView>
  );
}
