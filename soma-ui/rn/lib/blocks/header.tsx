import { View } from 'react-native';
import { Text, Button } from '@/lib/components';

export type HeaderProps = {
  brand: string;
  actions?: { label: string; variant?: 'default' | 'outline' | 'ghost'; onPress: () => void }[];
};

/** Top app/marketing header: brand left, action buttons right. */
export function Header({ brand, actions = [] }: HeaderProps) {
  return (
    <View className="flex-row items-center justify-between border-b border-border bg-card px-4 py-3">
      <Text className="font-heading-bold text-lg text-foreground">{brand}</Text>
      <View className="flex-row items-center gap-2">
        {actions.map((a) => (
          <Button key={a.label} size="sm" variant={a.variant ?? 'ghost'} label={a.label} onPress={a.onPress} />
        ))}
      </View>
    </View>
  );
}
