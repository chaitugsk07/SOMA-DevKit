import { useState } from 'react';
import { View } from 'react-native';
import { ComponentPage, Demo } from '../component-page';
import { FadeIn, SlideIn, AnimateGroup, PressableScale, Shimmer, Text, Card, Button } from '@/lib/components';

// Remounting via a key replays the entrance animations on demand.
function Replay({ render }: { render: (k: number) => React.ReactNode }) {
  const [k, setK] = useState(0);
  return (
    <View className="w-full gap-3">
      <View key={k}>{render(k)}</View>
      <Button size="sm" variant="outline" label="Replay" onPress={() => setK((n) => n + 1)} />
    </View>
  );
}

export function FadeInScreen() {
  return (
    <ComponentPage title="FadeIn" description="Fades + lifts content in on mount.">
      <Demo label="Default">
        <Replay render={() => <FadeIn><Card className="p-4"><Text variant="p">I faded in.</Text></Card></FadeIn>} />
      </Demo>
    </ComponentPage>
  );
}

export function SlideInScreen() {
  return (
    <ComponentPage title="SlideIn" description="Slides + fades content in from an edge.">
      <Demo label="From left">
        <Replay render={() => <SlideIn from="left"><Card className="p-4"><Text variant="p">Slid in from the left.</Text></Card></SlideIn>} />
      </Demo>
    </ComponentPage>
  );
}

export function AnimateGroupScreen() {
  return (
    <ComponentPage title="AnimateGroup" description="Staggered entrance for lists and grids.">
      <Demo label="Staggered">
        <Replay
          render={() => (
            <AnimateGroup className="gap-2" stagger={100}>
              {['One', 'Two', 'Three', 'Four'].map((t) => (
                <Card key={t} className="p-3"><Text variant="p">{t}</Text></Card>
              ))}
            </AnimateGroup>
          )}
        />
      </Demo>
    </ComponentPage>
  );
}

export function PressableScaleScreen() {
  return (
    <ComponentPage title="PressableScale" description="Springs down on press — tactile feel.">
      <Demo label="Press and hold">
        <PressableScale className="rounded-lg bg-primary px-5 py-3">
          <Text className="font-body-medium text-sm text-primary-foreground">Press me</Text>
        </PressableScale>
      </Demo>
    </ComponentPage>
  );
}

export function ShimmerScreen() {
  return (
    <ComponentPage title="Shimmer" description="Looping opacity pulse for loading states.">
      <Demo label="Placeholders">
        <View className="w-full gap-3">
          <Shimmer className="h-10 w-10 rounded-full" />
          <Shimmer className="h-4 w-3/4" />
          <Shimmer className="h-4 w-1/2" />
        </View>
      </Demo>
    </ComponentPage>
  );
}
