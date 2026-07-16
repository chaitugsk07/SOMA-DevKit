import { ScrollView, View } from 'react-native';
import { Text, Card, CardHeader, CardTitle, CardDescription } from '@/lib/components';
import { REGISTRY } from './registry';

const total = REGISTRY.reduce((n, c) => n + c.entries.length, 0);

export function Home() {
  return (
    <ScrollView className="flex-1 bg-background" contentContainerClassName="p-4 md:p-8 gap-8">
      <View className="max-w-3xl gap-1">
        <Text variant="h1">soma-ui</Text>
        <Text variant="muted">
          React Native components in the Palantir design language — {total} components across{' '}
          {REGISTRY.length} categories. Pick one from the sidebar.
        </Text>
      </View>
      <View className="max-w-3xl flex-row flex-wrap gap-3">
        {REGISTRY.map((cat) => (
          <Card key={cat.category} className="min-w-[160px] flex-1">
            <CardHeader>
              <CardTitle>{cat.category}</CardTitle>
              <CardDescription>{cat.entries.map((e) => e.name).join(' · ')}</CardDescription>
            </CardHeader>
          </Card>
        ))}
      </View>
    </ScrollView>
  );
}
