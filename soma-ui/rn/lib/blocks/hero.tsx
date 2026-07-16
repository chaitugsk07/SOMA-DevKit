import { View } from 'react-native';
import { Text, Button } from '@/lib/components';
import { FadeIn } from '@/lib/components/motion';

export type HeroProps = {
  title: string;
  subtitle?: string;
  cta?: { label: string; onPress: () => void };
  secondaryCta?: { label: string; onPress: () => void };
};

/** Centered hero section: headline, subtitle, CTAs — animated in. */
export function Hero({ title, subtitle, cta, secondaryCta }: HeroProps) {
  return (
    <FadeIn>
      <View className="items-center gap-4 bg-background px-6 py-16">
        <Text variant="h1" className="text-center">{title}</Text>
        {subtitle && (
          <Text variant="muted" className="max-w-md text-center text-base">{subtitle}</Text>
        )}
        {(cta || secondaryCta) && (
          <View className="mt-2 flex-row flex-wrap items-center justify-center gap-3">
            {cta && <Button label={cta.label} size="lg" onPress={cta.onPress} />}
            {secondaryCta && (
              <Button variant="outline" size="lg" label={secondaryCta.label} onPress={secondaryCta.onPress} />
            )}
          </View>
        )}
      </View>
    </FadeIn>
  );
}
