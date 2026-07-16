import { View, ScrollView, Pressable } from 'react-native';
import { MotiView } from 'moti';
import { Text } from '@/lib/components';

/** V1 — The confirmation moment. Oversized thank-you + hairline receipt. */
export function LuxeCheckoutV1() {
  return (
    <ScrollView className="flex-1 bg-background" contentContainerClassName="flex-grow items-center pb-10 pt-16">
      <View className="w-full max-w-3xl px-6">
        {/* Animated seal */}
        <View className="items-center">
          <MotiView from={{ scale: 0.4, opacity: 0 }} animate={{ scale: 1, opacity: 1 }} transition={{ type: 'spring', damping: 12, stiffness: 180 }}>
            <View className="h-20 w-20 items-center justify-center rounded-full border-2 border-primary">
              <Text className="text-3xl text-primary">✓</Text>
            </View>
          </MotiView>
        </View>

        <MotiView from={{ opacity: 0, translateY: 16 }} animate={{ opacity: 1, translateY: 0 }} transition={{ type: 'timing', duration: 500, delay: 250 }} className="mt-8 items-center">
          <Text className="text-xs uppercase tracking-[4px] text-muted-foreground">Order confirmed</Text>
          <Text className="mt-2 text-center font-heading-bold text-6xl leading-[0.92] text-foreground">Thank{'\n'}you.</Text>
          <Text className="mt-4 text-center text-sm text-muted-foreground">
            Order #SOMA-4821 · A receipt is on its way to jane@example.com
          </Text>
        </MotiView>

        {/* Hairline receipt */}
        <MotiView from={{ opacity: 0 }} animate={{ opacity: 1 }} transition={{ type: 'timing', duration: 500, delay: 450 }} className="mt-10">
          {[['Orbit — Wireless Headphones', '$349'], ['Ceramic — Pour-Over Set', '$96'], ['Shipping', '$12']].map(([k, v], i) => (
            <View key={k} className={'flex-row justify-between py-4 ' + (i === 0 ? 'border-y border-border' : 'border-b border-border')}>
              <Text className="flex-1 font-body text-sm text-muted-foreground">{k}</Text>
              <Text className="font-body-medium text-sm text-foreground">{v}</Text>
            </View>
          ))}
          <View className="flex-row items-center justify-between py-5">
            <Text className="text-xs uppercase tracking-[3px] text-muted-foreground">Total</Text>
            <Text className="font-heading-bold text-3xl text-foreground">$457</Text>
          </View>
        </MotiView>

        <Pressable className="mt-6 items-center justify-center rounded-full bg-foreground py-4 active:opacity-80">
          <Text className="text-sm tracking-[2px] text-background">TRACK ORDER</Text>
        </Pressable>
        <Pressable className="mt-3 items-center py-2 active:opacity-60">
          <Text className="text-sm text-muted-foreground">Continue shopping</Text>
        </Pressable>
      </View>
    </ScrollView>
  );
}
