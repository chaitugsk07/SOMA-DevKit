import { useState } from 'react';
import { View, ScrollView } from 'react-native';
import { AnimatePresence, MotiView } from 'moti';
import {
  Text,
  Button,
  Card,
  CardContent,
  MapPlaceholder,
  StatusPill,
  OrderStatus,
  PriceTag,
  FadeIn,
  Reveal,
} from '@/lib/components';
import { ACTIVE_DELIVERY, DELIVERY_STEPS } from './data';

const ACTIONS = ['Arrived at pickup', 'Confirm pickup', 'Complete delivery', 'Delivered ✓'];

/** Active delivery: live map, route, progress timeline, advancing action button. */
export function ActiveScreen() {
  const [step, setStep] = useState(0);
  const done = step >= ACTIONS.length - 1;

  return (
    <View className="flex-1 bg-background">
      <ScrollView contentContainerClassName="items-center">
        <View className="w-full max-w-2xl p-4 gap-4 pb-28">
        <FadeIn>
          <MapPlaceholder eta="ETA 8 min · 3.2 km" className="h-44 w-full" />
        </FadeIn>

        <Reveal direction="up" delay={60}>
          <View className="flex-row items-center justify-between">
            <StatusPill
              label={done ? 'Delivered' : step >= 2 ? 'En route to dropoff' : 'En route to pickup'}
              tone={done ? 'success' : 'active'}
              live={!done}
            />
            <PriceTag amount={ACTIVE_DELIVERY.payout} size="md" />
          </View>
        </Reveal>

        <Reveal direction="up" delay={120}>
          <Card>
            <CardContent className="pt-4">
              <OrderStatus steps={DELIVERY_STEPS} current={step} />
            </CardContent>
          </Card>
        </Reveal>

        <Reveal direction="up" delay={180}>
          <Card>
            <CardContent className="gap-2 pt-4">
              <View>
                <Text variant="small" className="text-muted-foreground">Pickup</Text>
                <Text variant="p" className="font-body-medium">{ACTIVE_DELIVERY.pickup}</Text>
              </View>
              <View>
                <Text variant="small" className="text-muted-foreground">Dropoff</Text>
                <Text variant="p" className="font-body-medium">{ACTIVE_DELIVERY.dropoff}</Text>
              </View>
            </CardContent>
          </Card>
        </Reveal>
        </View>
      </ScrollView>

      {/* Sticky advancing action */}
      <View className="absolute inset-x-0 bottom-0 border-t border-border bg-card p-4">
        <Button
          size="lg"
          variant={done ? 'secondary' : 'default'}
          label={ACTIONS[step]}
          disabled={done}
          onPress={() => setStep((s) => Math.min(ACTIONS.length - 1, s + 1))}
        />
      </View>

      {/* Delivered celebration */}
      <AnimatePresence>
        {done && (
          <MotiView
            from={{ opacity: 0, scale: 0.8 }}
            animate={{ opacity: 1, scale: 1 }}
            exit={{ opacity: 0 }}
            transition={{ type: 'spring', damping: 14, stiffness: 200 }}
            className="absolute inset-x-0 top-6 items-center"
            pointerEvents="none"
          >
            <View className="rounded-full bg-success px-4 py-2">
              <Text className="font-body-semibold text-sm text-success-foreground">
                Delivered — {`$${ACTIVE_DELIVERY.payout.toFixed(2)}`} earned
              </Text>
            </View>
          </MotiView>
        )}
      </AnimatePresence>
    </View>
  );
}
