import { useState } from 'react';
import { View, ScrollView, Pressable } from 'react-native';
import {
  Text,
  Button,
  AvailabilityToggle,
  DeliveryCard,
  AnimateGroup,
  FadeIn,
} from '@/lib/components';
import { AVAILABLE_JOBS } from './data';

/** Driver home: availability toggle + queue of available delivery jobs. */
export function QueueScreen() {
  const [online, setOnline] = useState(true);
  const [accepted, setAccepted] = useState<string | null>(null);

  return (
    <ScrollView className="flex-1 bg-background" contentContainerClassName="items-center">
      <View className="w-full max-w-2xl p-4 gap-4">
      <FadeIn>
        <View>
          <Text variant="muted" className="text-sm">Driver</Text>
          <Text variant="h1">Deliveries</Text>
        </View>
      </FadeIn>

      <AvailabilityToggle online={online} onToggle={setOnline} />

      {online ? (
        <>
          <View className="flex-row items-center justify-between">
            <Text variant="p" className="font-body-semibold">Available now</Text>
            <Text variant="muted" className="text-sm">{AVAILABLE_JOBS.length} nearby</Text>
          </View>
          <AnimateGroup className="gap-3" stagger={80}>
            {AVAILABLE_JOBS.map((job) => (
              <Pressable key={job.id} onPress={() => setAccepted(job.id)}>
                <DeliveryCard
                  delivery={
                    accepted === job.id
                      ? { ...job, status: { label: 'Accepted', tone: 'success' } }
                      : job
                  }
                />
              </Pressable>
            ))}
          </AnimateGroup>
          {accepted && (
            <Button size="lg" label="Go to active delivery" />
          )}
        </>
      ) : (
        <View className="items-center py-16">
          <Text variant="muted">You're offline. Go online to see jobs.</Text>
        </View>
      )}
      </View>
    </ScrollView>
  );
}
