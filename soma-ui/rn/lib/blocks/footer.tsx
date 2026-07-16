import { View, Pressable } from 'react-native';
import { Text } from '@/lib/components';

export type FooterProps = {
  brand: string;
  links?: { label: string; onPress: () => void }[];
  note?: string;
};

/** Footer: brand, link row, fine-print note. */
export function Footer({ brand, links = [], note }: FooterProps) {
  return (
    <View className="gap-3 border-t border-border bg-card px-6 py-8">
      <Text className="font-heading-bold text-base text-foreground">{brand}</Text>
      {links.length > 0 && (
        <View className="flex-row flex-wrap gap-x-5 gap-y-2">
          {links.map((l) => (
            <Pressable key={l.label} onPress={l.onPress} className="active:opacity-70">
              <Text variant="muted" className="text-sm">{l.label}</Text>
            </Pressable>
          ))}
        </View>
      )}
      {note && <Text variant="small" className="text-muted-foreground">{note}</Text>}
    </View>
  );
}
