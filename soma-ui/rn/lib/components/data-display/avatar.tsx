import { View, Image, Text } from 'react-native';
import { cn } from '@/lib/utils/cn';

const sizes = {
  sm: 'h-8 w-8',
  default: 'h-10 w-10',
  lg: 'h-14 w-14',
} as const;

export type AvatarProps = {
  uri?: string;
  fallback?: string;
  size?: keyof typeof sizes;
  className?: string;
};

export function Avatar({ uri, fallback, size = 'default', className }: AvatarProps) {
  return (
    <View
      className={cn(
        'items-center justify-center overflow-hidden rounded-full bg-muted',
        sizes[size],
        className,
      )}
    >
      {uri ? (
        <Image source={{ uri }} className="h-full w-full" />
      ) : (
        <Text className="font-body-medium text-sm text-muted-foreground">
          {(fallback ?? '?').slice(0, 2).toUpperCase()}
        </Text>
      )}
    </View>
  );
}
