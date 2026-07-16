import { useState } from 'react';
import { View, ScrollView } from 'react-native';
import { AnimatePresence, MotiView } from 'moti';
import {
  Text,
  Input,
  Button,
  Card,
  CardContent,
  PriceTag,
  FadeIn,
  Reveal,
} from '@/lib/components';

/** Checkout: address + payment cards, sticky place-order, animated success. */
export function CheckoutScreen() {
  const [placed, setPlaced] = useState(false);
  const total = 537.0;

  return (
    <View className="flex-1 bg-background">
      <ScrollView contentContainerClassName="pb-32 items-center">
        <View className="w-full max-w-2xl p-4 gap-4">
          <FadeIn>
            <Text variant="h1">Checkout</Text>
          </FadeIn>

          <Reveal direction="up" delay={60}>
            <Card>
              <CardContent className="gap-3 pt-4">
                <Text variant="p" className="font-body-semibold">Shipping address</Text>
                <Input placeholder="Full name" />
                <Input placeholder="Street address" />
                <View className="flex-row gap-3">
                  <Input className="flex-1" placeholder="City" />
                  <Input className="w-24" placeholder="ZIP" keyboardType="number-pad" />
                </View>
              </CardContent>
            </Card>
          </Reveal>

          <Reveal direction="up" delay={140}>
            <Card>
              <CardContent className="gap-3 pt-4">
                <Text variant="p" className="font-body-semibold">Payment</Text>
                <Input placeholder="Card number" keyboardType="number-pad" />
                <View className="flex-row gap-3">
                  <Input className="flex-1" placeholder="MM / YY" />
                  <Input className="w-24" placeholder="CVC" keyboardType="number-pad" />
                </View>
              </CardContent>
            </Card>
          </Reveal>
        </View>
      </ScrollView>

      <View className="absolute inset-x-0 bottom-0 flex-row items-center gap-3 border-t border-border bg-card p-4">
        <View>
          <Text variant="small" className="text-muted-foreground">Total</Text>
          <PriceTag amount={total} size="md" />
        </View>
        <Button className="flex-1" size="lg" label="Place order" onPress={() => setPlaced(true)} />
      </View>

      {/* Success overlay */}
      <AnimatePresence>
        {placed && (
          <MotiView
            from={{ opacity: 0 }}
            animate={{ opacity: 1 }}
            exit={{ opacity: 0 }}
            className="absolute inset-0 items-center justify-center bg-background/95 p-6"
          >
            <MotiView
              from={{ scale: 0.6, opacity: 0 }}
              animate={{ scale: 1, opacity: 1 }}
              transition={{ type: 'spring', damping: 14, stiffness: 200 }}
              className="items-center gap-3"
            >
              <View className="h-16 w-16 items-center justify-center rounded-full bg-success">
                <Text className="text-2xl text-success-foreground">✓</Text>
              </View>
              <Text variant="h2">Order placed</Text>
              <Text variant="muted" className="text-center">Thanks — we'll email your receipt shortly.</Text>
              <Button variant="outline" label="Track order" onPress={() => setPlaced(false)} />
            </MotiView>
          </MotiView>
        )}
      </AnimatePresence>
    </View>
  );
}
