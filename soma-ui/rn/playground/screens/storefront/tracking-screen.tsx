import { useState } from 'react';
import { View, ScrollView, Image } from 'react-native';
import {
  Text,
  Button,
  Card,
  CardContent,
  Separator,
  OrderStatus,
  FadeIn,
  Reveal,
} from '@/lib/components';
import { PRODUCTS, ORDER_STEPS } from './data';

/** Order tracking: ETA, animated status timeline, order summary. */
export function TrackingScreen() {
  const [current, setCurrent] = useState(3); // "Out for delivery"
  const product = PRODUCTS[0];

  return (
    <ScrollView className="flex-1 bg-background" contentContainerClassName="items-center">
      <View className="w-full max-w-2xl p-4 gap-4">
        <FadeIn>
          <View>
            <Text variant="muted" className="text-sm">Order #SOMA-4821</Text>
            <Text variant="h1">Arriving today</Text>
            <Text variant="muted">Estimated 2:15 – 2:45 PM</Text>
          </View>
        </FadeIn>

        <Reveal direction="up" delay={60}>
          <Card>
            <CardContent className="pt-4">
              <OrderStatus steps={ORDER_STEPS} current={current} />
            </CardContent>
          </Card>
        </Reveal>

        <Reveal direction="up" delay={120}>
          <Card>
            <CardContent className="flex-row items-center gap-3 pt-4">
              <View className="h-14 w-14 overflow-hidden rounded-lg bg-muted">
                <Image source={{ uri: product.image }} className="h-full w-full" resizeMode="cover" />
              </View>
              <View className="flex-1">
                <Text variant="p" className="font-body-medium">{product.name}</Text>
                <Text variant="muted" className="text-sm">Qty 1 · ${product.price.toFixed(2)}</Text>
              </View>
            </CardContent>
          </Card>
        </Reveal>

        {/* Demo control to watch the timeline animate forward */}
        <Separator />
        <View className="flex-row items-center justify-between">
          <Text variant="muted" className="text-sm">Demo: advance status</Text>
          <View className="flex-row gap-2">
            <Button size="sm" variant="outline" label="Back" onPress={() => setCurrent((c) => Math.max(0, c - 1))} />
            <Button size="sm" label="Advance" onPress={() => setCurrent((c) => Math.min(ORDER_STEPS.length - 1, c + 1))} />
          </View>
        </View>
      </View>
    </ScrollView>
  );
}
