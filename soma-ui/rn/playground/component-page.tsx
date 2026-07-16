import { type ReactNode } from 'react';
import { ScrollView, View } from 'react-native';
import { Text } from '@/lib/components';

export function ComponentPage({
  title,
  description,
  children,
}: {
  title: string;
  description?: string;
  children: ReactNode;
}) {
  return (
    <ScrollView className="flex-1 bg-background" contentContainerClassName="p-4 gap-4 items-center">
      <View className="w-full max-w-3xl gap-4">
        <View className="gap-1">
          <Text variant="h2">{title}</Text>
          {description && <Text variant="muted">{description}</Text>}
        </View>
        {children}
      </View>
    </ScrollView>
  );
}

/** Labeled preview row — groups variant examples under a caption. */
export function Demo({ label, children }: { label: string; children: ReactNode }) {
  return (
    <View className="gap-2">
      <Text variant="small" className="text-muted-foreground">{label}</Text>
      <View className="flex-row flex-wrap items-center gap-3">{children}</View>
    </View>
  );
}
