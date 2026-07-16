import { View } from 'react-native';
import { Text, Card, CardHeader, CardTitle, CardDescription } from '@/lib/components';
import { AnimateGroup } from '@/lib/components/motion';

export type Feature = { title: string; description: string };

export type FeatureGridProps = {
  heading?: string;
  features: Feature[];
};

/** Responsive feature card grid with a staggered entrance. */
export function FeatureGrid({ heading, features }: FeatureGridProps) {
  return (
    <View className="gap-6 bg-background px-6 py-12">
      {heading && <Text variant="h2" className="text-center">{heading}</Text>}
      <AnimateGroup
        className="flex-row flex-wrap justify-center gap-4"
        itemClassName="min-w-[200px] flex-1"
        stagger={80}
      >
        {features.map((f) => (
          <Card key={f.title}>
            <CardHeader>
              <CardTitle>{f.title}</CardTitle>
              <CardDescription>{f.description}</CardDescription>
            </CardHeader>
          </Card>
        ))}
      </AnimateGroup>
    </View>
  );
}
