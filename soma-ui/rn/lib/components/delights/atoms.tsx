import { View, Pressable, type ViewProps } from 'react-native';
import { Text } from '@/lib/components/data-display/text';
import { productAccents, type ProductAccent } from '@/lib/theme/tokens';
import { cn } from '@/lib/utils/cn';

/** Small uppercase warm-grey label above a section heading (the ✦ overline). */
export function SectionOverline({ children, className }: { children: string; className?: string }) {
  return (
    <Text className={cn('font-body text-xs uppercase tracking-[3px] text-muted-foreground', className)}>
      ✦ {children}
    </Text>
  );
}

/** Editorial serif pull-quote (the "Did You Know" / fun-fact voice). */
export function PullQuote({ children, source, className }: { children: string; source?: string; className?: string }) {
  return (
    <View className={cn('gap-2 py-2', className)}>
      <Text className="font-serif text-xl italic leading-8 text-card-foreground">{children}</Text>
      {source && <Text className="font-body text-xs text-muted-foreground">— {source}</Text>}
    </View>
  );
}

/** The one splash of SKU color allowed — a product-identity dot. */
export function ProductAccentDot({ accent, size = 12, className }: { accent: ProductAccent; size?: number; className?: string }) {
  return (
    <View
      className={cn('rounded-full', className)}
      style={{ width: size, height: size, backgroundColor: productAccents[accent] }}
    />
  );
}

/** WhatsApp CTA — the primary channel at launch. Near-black per brand (no green). */
export function WhatsAppCTAButton({ label = 'Chat on WhatsApp', onPress, className }: { label?: string; onPress?: () => void; className?: string }) {
  return (
    <Pressable
      onPress={onPress}
      accessibilityRole="button"
      className={cn('flex-row items-center justify-center gap-2 rounded-full bg-primary px-5 py-3.5 active:opacity-85', className)}
    >
      <Text className="font-body-medium text-sm text-primary-foreground">{label}</Text>
      <Text className="text-primary-foreground">›</Text>
    </Pressable>
  );
}

/** Warm-grey card surface used across s-delights (no border, soft radius). */
export function DelightsCard({ className, ...props }: ViewProps & { className?: string }) {
  return <View className={cn('rounded-xl bg-card p-4', className)} {...props} />;
}
