import { type ReactNode } from 'react';
import { View, Pressable, Image } from 'react-native';
import { Text } from '@/lib/components/data-display/text';
import { cn } from '@/lib/utils/cn';

export type DataRowProps = {
  /** Leading avatar/thumbnail image, or a single-letter fallback is derived from title. */
  image?: string;
  title: string;
  subtitle?: string;
  /** Right-aligned primary value (e.g. price, count). */
  value?: string;
  /** Right-aligned secondary (e.g. status pill passed as node). */
  trailing?: ReactNode;
  onPress?: () => void;
  className?: string;
};

/** Generic admin list row: thumb, title/subtitle, value + trailing. Orders, products, customers. */
export function DataRow({ image, title, subtitle, value, trailing, onPress, className }: DataRowProps) {
  const Container = onPress ? Pressable : View;
  return (
    <Container onPress={onPress} className={cn('flex-row items-center gap-3 py-3', onPress && 'active:opacity-60', className)}>
      <View className="h-10 w-10 items-center justify-center overflow-hidden rounded-lg bg-muted">
        {image ? (
          <Image source={{ uri: image }} className="h-full w-full" resizeMode="cover" />
        ) : (
          <Text className="font-heading-semibold text-sm text-muted-foreground">{title.charAt(0)}</Text>
        )}
      </View>
      <View className="flex-1">
        <Text variant="p" numberOfLines={1} className="text-sm font-body-medium">{title}</Text>
        {subtitle && <Text variant="muted" numberOfLines={1} className="text-xs">{subtitle}</Text>}
      </View>
      <View className="items-end gap-0.5">
        {value && <Text variant="p" className="text-sm font-body-semibold">{value}</Text>}
        {trailing}
      </View>
    </Container>
  );
}
