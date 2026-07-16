import { useState, type ReactNode } from 'react';
import { View, Image, Pressable, ScrollView } from 'react-native';
import { MotiView } from 'moti';
import { Text } from '@/lib/components/data-display/text';
import { SectionOverline, PullQuote, ProductAccentDot } from './atoms';
import { type ProductAccent } from '@/lib/theme/tokens';
import { cn } from '@/lib/utils/cn';

// ─── IngredientCard ──────────────────────────────────────────────────────────
export function IngredientCard({ name, oneLiner, image, onPress, className }: {
  name: string; oneLiner: string; image?: string; onPress?: () => void; className?: string;
}) {
  return (
    <Pressable onPress={onPress} className={cn('gap-2 rounded-xl bg-secondary p-3', className)}>
      <View className="aspect-square w-full items-center justify-center overflow-hidden rounded-lg bg-muted">
        {image ? <Image source={{ uri: image }} className="h-full w-full" resizeMode="cover" />
               : <Text className="font-serif text-3xl text-muted-foreground">{name.charAt(0)}</Text>}
      </View>
      <Text className="font-heading-semibold text-base text-card-foreground">{name}</Text>
      <Text className="font-body text-xs leading-4 text-muted-foreground">{oneLiner}</Text>
    </Pressable>
  );
}

// ─── IngredientCardFlip ──────────────────────────────────────────────────────
/** Collectible card that flips front↔back (tap). Front: illustration + hook.
 *  Back: benefits + tip. Golden Edition shimmer for rares. */
export function IngredientCardFlip({ name, hook, cardNo, total, benefits, golden, image }: {
  name: string; hook: string; cardNo: number; total: number; benefits: string[]; golden?: boolean; image?: string;
}) {
  const [flipped, setFlipped] = useState(false);
  return (
    <Pressable onPress={() => setFlipped((f) => !f)} className="aspect-[3/4] w-56 self-center">
      <MotiView
        animate={{ rotateY: flipped ? '180deg' : '0deg' }}
        transition={{ type: 'timing', duration: 400 }}
        style={{ flex: 1 }}
        className={cn('overflow-hidden rounded-2xl border p-4', golden ? 'border-2 border-primary bg-secondary' : 'border-border bg-card')}
      >
        {!flipped ? (
          <View className="flex-1 gap-3">
            <View className="flex-row items-center justify-between">
              {golden && <SectionOverline>Golden Edition</SectionOverline>}
              <Text className="ml-auto font-body text-xs text-muted-foreground">{cardNo} / {total}</Text>
            </View>
            <View className="flex-1 items-center justify-center overflow-hidden rounded-lg bg-muted">
              {image ? <Image source={{ uri: image }} className="h-full w-full" resizeMode="cover" />
                     : <Text className="font-serif text-5xl text-muted-foreground">{name.charAt(0)}</Text>}
            </View>
            <Text className="font-heading-bold text-2xl text-card-foreground">{name}</Text>
            <Text className="font-serif text-sm italic text-muted-foreground">{hook}</Text>
          </View>
        ) : (
          <View className="flex-1 gap-2" style={{ transform: [{ scaleX: -1 }] }}>
            <Text className="font-heading-semibold text-lg text-card-foreground">{name}</Text>
            {benefits.map((b) => (
              <View key={b} className="flex-row gap-2">
                <Text className="text-muted-foreground">✦</Text>
                <Text className="flex-1 font-body text-sm text-card-foreground">{b}</Text>
              </View>
            ))}
          </View>
        )}
      </MotiView>
    </Pressable>
  );
}

// ─── SKUBottlePageLayout ─────────────────────────────────────────────────────
/** The QR-linked bottle page: hero, ingredients, why-it-works, science, best-consumed. */
export function SKUBottlePageLayout({ name, tagline, accent, image, ingredients, why, science, bestConsumed, funFact, funFactSource, children }: {
  name: string;
  tagline: string;
  accent: ProductAccent;
  image?: string;
  ingredients: { name: string; grams: string; source?: string }[];
  why: string[];
  science: string;
  bestConsumed: { icon: string; label: string }[];
  funFact?: string;
  funFactSource?: string;
  children?: ReactNode;
}) {
  return (
    <ScrollView className="flex-1 bg-background" contentContainerClassName="pb-12">
      {/* Hero */}
      <View className="h-72 w-full items-center justify-center bg-muted">
        {image ? <Image source={{ uri: image }} className="h-full w-full" resizeMode="cover" />
               : <ProductAccentDot accent={accent} size={72} />}
      </View>
      <View className="gap-6 p-5">
        <View className="flex-row items-center gap-2">
          <ProductAccentDot accent={accent} size={14} />
          <View className="flex-1">
            <Text className="font-heading-bold text-3xl text-card-foreground">{name}</Text>
            <Text className="font-body text-sm text-muted-foreground">{tagline}</Text>
          </View>
        </View>

        {/* Ingredients */}
        <View className="gap-2">
          <SectionOverline>Inside this bottle</SectionOverline>
          {ingredients.map((ing) => (
            <View key={ing.name} className="flex-row items-baseline justify-between border-b border-border py-2">
              <Text className="font-body-medium text-sm text-card-foreground">{ing.name}</Text>
              <View className="flex-row items-baseline gap-2">
                {ing.source && <Text className="font-body text-xs text-muted-foreground">{ing.source}</Text>}
                <Text className="font-body text-sm text-muted-foreground">{ing.grams}</Text>
              </View>
            </View>
          ))}
        </View>

        {/* Why it works */}
        <View className="gap-2">
          <SectionOverline>Why this works</SectionOverline>
          {why.map((w) => (
            <View key={w} className="flex-row gap-2">
              <Text className="text-muted-foreground">✦</Text>
              <Text className="flex-1 font-body text-sm leading-5 text-card-foreground">{w}</Text>
            </View>
          ))}
        </View>

        {/* Science */}
        <View className="gap-2">
          <SectionOverline>The science</SectionOverline>
          <Text className="font-body text-sm leading-6 text-muted-foreground">{science}</Text>
        </View>

        {/* Best consumed */}
        <View className="flex-row flex-wrap gap-4">
          {bestConsumed.map((b) => (
            <View key={b.label} className="items-center gap-1">
              <Text className="text-xl text-muted-foreground">{b.icon}</Text>
              <Text className="font-body text-xs text-muted-foreground">{b.label}</Text>
            </View>
          ))}
        </View>

        {funFact && <PullQuote source={funFactSource}>{funFact}</PullQuote>}
        {children}
      </View>
    </ScrollView>
  );
}
