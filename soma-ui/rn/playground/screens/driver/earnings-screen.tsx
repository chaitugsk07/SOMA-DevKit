import { View, ScrollView } from 'react-native';
import {
  Text,
  Card,
  CardContent,
  Separator,
  EarningsStat,
  AnimatedRing,
  PriceTag,
  FadeIn,
  Reveal,
  AnimateGroup,
} from '@/lib/components';

const RECENT = [
  { id: 'r1', to: '128 Birch Lane', time: '2:24 PM', amount: 8.5 },
  { id: 'r2', to: '77 Cedar Ct', time: '1:58 PM', amount: 12.0 },
  { id: 'r3', to: '901 Elm Ave', time: '1:30 PM', amount: 6.75 },
  { id: 'r4', to: '15 Willow Rd', time: '12:55 PM', amount: 15.25 },
];

/** Driver earnings: animated stat tiles, acceptance ring, recent deliveries. */
export function EarningsScreen() {
  return (
    <ScrollView className="flex-1 bg-background" contentContainerClassName="items-center">
      <View className="w-full max-w-2xl p-4 gap-4">
      <FadeIn>
        <View>
          <Text variant="muted" className="text-sm">Today</Text>
          <Text variant="h1">Earnings</Text>
        </View>
      </FadeIn>

      <Reveal direction="up" delay={60}>
        <View className="flex-row gap-3">
          <EarningsStat className="flex-1" label="Today" value={142.5} prefix="$" decimals={2} />
          <EarningsStat className="flex-1" label="Deliveries" value={11} />
        </View>
      </Reveal>

      <Reveal direction="up" delay={120}>
        <Card>
          <CardContent className="flex-row items-center gap-4 pt-4">
            <AnimatedRing value={96} size={80} strokeWidth={7} />
            <View className="flex-1">
              <Text variant="p" className="font-body-semibold">Acceptance rate</Text>
              <Text variant="muted" className="text-sm">Great work — keep it above 90% for bonuses.</Text>
            </View>
          </CardContent>
        </Card>
      </Reveal>

      <Reveal direction="up" delay={180}>
        <Text variant="p" className="font-body-semibold">Recent deliveries</Text>
      </Reveal>
      <AnimateGroup className="gap-0" stagger={60}>
        {RECENT.map((r, i) => (
          <View key={r.id}>
            {i > 0 && <Separator />}
            <View className="flex-row items-center justify-between py-3">
              <View>
                <Text variant="p" className="text-sm font-body-medium">{r.to}</Text>
                <Text variant="muted" className="text-xs">{r.time}</Text>
              </View>
              <PriceTag amount={r.amount} size="sm" />
            </View>
          </View>
        ))}
      </AnimateGroup>
      </View>
    </ScrollView>
  );
}
