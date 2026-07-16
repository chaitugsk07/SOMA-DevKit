import { View, ScrollView, Pressable } from 'react-native';
import { Text, Avatar, Card, CardContent, Separator, Badge, FadeIn, Reveal } from '@/lib/components';

const MENU = [
  { icon: '❏', label: 'My orders', sub: '11 orders' },
  { icon: '♥', label: 'Wishlist', sub: '4 saved' },
  { icon: '⌂', label: 'Addresses', sub: '2 saved' },
  { icon: '▤', label: 'Payment methods', sub: 'Visa •••• 4242' },
  { icon: '✷', label: 'Notifications' },
  { icon: '?', label: 'Help & support' },
];

/** Customer account: profile header, membership, settings menu. */
export function AccountScreen() {
  return (
    <ScrollView className="flex-1 bg-background" contentContainerClassName="p-4 gap-4">
      <FadeIn>
        <View className="flex-row items-center gap-3">
          <Avatar size="lg" fallback="JD" />
          <View className="flex-1">
            <Text variant="h2">Jane Doe</Text>
            <Text variant="muted" className="text-sm">jane@example.com</Text>
          </View>
          <Badge variant="secondary" label="Member" />
        </View>
      </FadeIn>

      <Reveal direction="up" delay={60}>
        <Card>
          <CardContent className="flex-row justify-around pt-4">
            <Stat value="11" label="Orders" />
            <Separator orientation="vertical" />
            <Stat value="$1.2k" label="Spent" />
            <Separator orientation="vertical" />
            <Stat value="240" label="Points" />
          </CardContent>
        </Card>
      </Reveal>

      <Reveal direction="up" delay={120}>
        <Card>
          <CardContent className="pt-0">
            {MENU.map((m, i) => (
              <View key={m.label}>
                {i > 0 && <Separator />}
                <Pressable className="flex-row items-center gap-3 py-3.5 active:opacity-60">
                  <Text className="w-6 text-center text-lg text-muted-foreground">{m.icon}</Text>
                  <View className="flex-1">
                    <Text variant="p" className="text-sm font-body-medium">{m.label}</Text>
                    {m.sub && <Text variant="muted" className="text-xs">{m.sub}</Text>}
                  </View>
                  <Text className="text-muted-foreground">›</Text>
                </Pressable>
              </View>
            ))}
          </CardContent>
        </Card>
      </Reveal>

      <Text variant="muted" className="py-2 text-center text-xs">soma store · v1.0.0</Text>
    </ScrollView>
  );
}

function Stat({ value, label }: { value: string; label: string }) {
  return (
    <View className="items-center gap-0.5">
      <Text variant="h3">{value}</Text>
      <Text variant="muted" className="text-xs">{label}</Text>
    </View>
  );
}
