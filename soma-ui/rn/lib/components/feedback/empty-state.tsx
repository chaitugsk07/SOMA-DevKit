import { type ReactNode } from 'react';
import { View } from 'react-native';
import { Text } from '@/lib/components/data-display/text';
import { Button } from '@/lib/components/inputs/button';
import { cn } from '@/lib/utils/cn';

export type EmptyStateProps = {
  /** Optional icon rendered in a soft circle. Pass a lucide-react-native icon at ~28px. */
  icon?: ReactNode;
  /** Short heading — what is empty. */
  title: string;
  /** One or two warm sentences explaining the empty state or what to do next. */
  subtitle?: string;
  /** CTA button label. Requires `onAction`. */
  actionLabel?: string;
  /** Called when the CTA button is tapped. */
  onAction?: () => void;
  className?: string;
};

/**
 * Designed empty state — feels inviting, not like an error.
 *
 * Usage:
 *   <EmptyState icon={<Leaf size={28} />} title="No juices yet" subtitle="Your plan will appear here once it's set up." actionLabel="Browse juices" onAction={...} />
 */
export function EmptyState({
  icon,
  title,
  subtitle,
  actionLabel,
  onAction,
  className,
}: EmptyStateProps) {
  return (
    <View
      className={cn('flex-1 items-center justify-center px-8 py-16', className)}
    >
      {icon != null && (
        <View
          className="mb-5 h-16 w-16 items-center justify-center rounded-full bg-secondary"
          accessibilityElementsHidden
        >
          {icon}
        </View>
      )}

      <Text className="font-heading-semibold text-lg text-foreground text-center">
        {title}
      </Text>

      {subtitle != null && (
        <Text
          className="mt-2 font-body text-sm text-muted-foreground text-center"
          style={{ maxWidth: 260 }}
        >
          {subtitle}
        </Text>
      )}

      {actionLabel != null && onAction != null && (
        <View className="mt-6">
          <Button
            label={actionLabel}
            onPress={onAction}
            variant="secondary"
          />
        </View>
      )}
    </View>
  );
}
