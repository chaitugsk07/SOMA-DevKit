import { useState } from 'react';
import { View } from 'react-native';
import { ComponentPage, Demo } from '../component-page';
import {
  CountUp,
  Reveal,
  AnimatedRing,
  Typewriter,
  AnimatedNumber,
  Pulse,
  AnimatedList,
  AnimatedListItem,
  Text,
  Card,
  Button,
  Badge,
} from '@/lib/components';

// Remounting via a key replays mount-driven animations on demand.
function Replay({ render }: { render: (k: number) => React.ReactNode }) {
  const [k, setK] = useState(0);
  return (
    <View className="w-full items-start gap-3">
      <View key={k}>{render(k)}</View>
      <Button size="sm" variant="outline" label="Replay" onPress={() => setK((n) => n + 1)} />
    </View>
  );
}

export function CountUpScreen() {
  return (
    <ComponentPage title="CountUp" description="Counts a number up on mount — for stats and dashboards.">
      <Demo label="Stats">
        <Replay
          render={() => (
            <View className="flex-row gap-6">
              <View className="items-center">
                <CountUp to={1284} variant="h1" />
                <Text variant="muted" className="text-xs">Users</Text>
              </View>
              <View className="items-center">
                <CountUp to={99.9} decimals={1} suffix="%" variant="h1" />
                <Text variant="muted" className="text-xs">Uptime</Text>
              </View>
            </View>
          )}
        />
      </Demo>
    </ComponentPage>
  );
}

export function RevealScreen() {
  return (
    <ComponentPage title="Reveal" description="Animates a section in from a direction.">
      <Demo label="Directions">
        <Replay
          render={() => (
            <View className="w-full gap-3">
              <Reveal direction="up"><Card className="p-4"><Text variant="p">Up</Text></Card></Reveal>
              <Reveal direction="left" delay={100}><Card className="p-4"><Text variant="p">Left</Text></Card></Reveal>
              <Reveal direction="right" delay={200}><Card className="p-4"><Text variant="p">Right</Text></Card></Reveal>
            </View>
          )}
        />
      </Demo>
    </ComponentPage>
  );
}

export function AnimatedRingScreen() {
  return (
    <ComponentPage title="AnimatedRing" description="Circular progress ring that fills on mount.">
      <Demo label="Scores">
        <Replay
          render={() => (
            <View className="flex-row gap-6">
              <AnimatedRing value={72} />
              <AnimatedRing value={94} size={96} />
            </View>
          )}
        />
      </Demo>
    </ComponentPage>
  );
}

export function TypewriterScreen() {
  return (
    <ComponentPage title="Typewriter" description="Types text out character by character.">
      <Demo label="Default">
        <Replay render={() => <Typewriter text="Build it once. Ship everywhere." variant="h3" />} />
      </Demo>
    </ComponentPage>
  );
}

export function AnimatedNumberScreen() {
  const [value, setValue] = useState(50);
  return (
    <ComponentPage title="AnimatedNumber" description="Smoothly tweens whenever the value changes.">
      <Demo label="Live counter">
        <View className="w-full items-start gap-3">
          <AnimatedNumber value={value} variant="h1" />
          <View className="flex-row gap-2">
            <Button size="sm" variant="outline" label="-25" onPress={() => setValue((v) => v - 25)} />
            <Button size="sm" variant="outline" label="+25" onPress={() => setValue((v) => v + 25)} />
            <Button size="sm" variant="outline" label="Random" onPress={() => setValue(Math.round(Math.random() * 1000))} />
          </View>
        </View>
      </Demo>
    </ComponentPage>
  );
}

export function PulseScreen() {
  return (
    <ComponentPage title="Pulse" description="Looping scale pulse to draw attention.">
      <Demo label="Attention">
        <Pulse><Badge variant="destructive" label="LIVE" /></Pulse>
      </Demo>
    </ComponentPage>
  );
}

export function AnimatedListScreen() {
  const [items, setItems] = useState(['Alpha', 'Bravo', 'Charlie']);
  const add = () => setItems((cur) => [...cur, `Item ${cur.length + 1}`]);
  const remove = (t: string) => setItems((cur) => cur.filter((x) => x !== t));
  return (
    <ComponentPage title="AnimatedList" description="Items animate in/out and reflow on add, remove, reorder.">
      <Demo label="Add / remove">
        <View className="w-full gap-3">
          <Button size="sm" label="Add item" onPress={add} />
          <AnimatedList className="gap-2">
            {items.map((t) => (
              <AnimatedListItem key={t}>
                <Card className="flex-row items-center justify-between p-3">
                  <Text variant="p">{t}</Text>
                  <Button size="sm" variant="ghost" label="Remove" onPress={() => remove(t)} />
                </Card>
              </AnimatedListItem>
            ))}
          </AnimatedList>
        </View>
      </Demo>
    </ComponentPage>
  );
}
