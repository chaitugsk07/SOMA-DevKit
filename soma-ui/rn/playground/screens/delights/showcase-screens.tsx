import { useState } from 'react';
import { View, ScrollView } from 'react-native';
import { SomaDelightsThemeProvider } from '@/lib/theme';
import { Columns } from '@/lib/hooks';
import {
  Text,
  SectionOverline,
  PlanCard,
  JuiceCard,
  AnchoredPriceDisplay,
  IngredientCard,
  IngredientCardFlip,
  SKUBottlePageLayout,
  TwentyOneDayGrid,
  StreakBadge,
  WellnessQuizStepper,
  WhatsAppCTAButton,
  type Plan,
} from '@/lib/components';

const wrap = (node: React.ReactNode) => <SomaDelightsThemeProvider>{node}</SomaDelightsThemeProvider>;

// ── SKU bottle page (the QR education hero) ──
export function DelightsSkuScreen() {
  return wrap(
    <View className="flex-1 items-center">
      <View className="w-full max-w-3xl flex-1">
        <SKUBottlePageLayout
          name="Green Vitality"
          tagline="Cold-pressed spinach, cucumber & mint"
          accent="vitalityGreen"
          image="https://images.unsplash.com/photo-1610970881699-44a5587cabec?w=900&q=90"
          ingredients={[
            { name: 'Spinach', grams: '80g', source: 'Bowenpally mkt' },
            { name: 'Cucumber', grams: '120g', source: 'Bowenpally mkt' },
            { name: 'Green apple', grams: '60g' },
            { name: 'Mint', grams: '8g' },
            { name: 'Lemon', grams: '10g' },
          ]}
          why={[
            'Spinach delivers plant iron; the lemon’s vitamin C boosts its absorption.',
            'Cucumber is 95% water — gentle hydration on an empty stomach.',
            'Mint calms digestion and lifts the taste without any added sugar.',
          ]}
          science="Non-heme (plant) iron absorbs far better alongside vitamin C — pairing spinach with lemon can multiply uptake several times over. Pressed cold, the enzymes stay intact."
          bestConsumed={[
            { icon: '○', label: 'Morning' },
            { icon: '❄', label: 'Chilled' },
            { icon: '∅', label: 'Empty stomach' },
          ]}
          funFact="A single cold-pressed bottle uses nearly half a kilo of produce — more than most people eat in a day."
          funFactSource="Soma kitchen"
        >
          <WhatsAppCTAButton label="Order Green Vitality" className="mt-2" />
        </SKUBottlePageLayout>
      </View>
    </View>,
  );
}

// ── Plans ──
const PLANS: Plan[] = [
  { name: 'Daily', monthly: 4500, perDay: 150, bottlesPerWeek: 7, deliveryDays: 'every day', featured: true },
  { name: 'Weekday', monthly: 3300, perDay: 165, bottlesPerWeek: 5, deliveryDays: 'Mon–Fri' },
  { name: 'Starter', monthly: 1200, perDay: 200, bottlesPerWeek: 3, deliveryDays: 'Mon/Wed/Fri' },
];
export function DelightsPlansScreen() {
  const [sel, setSel] = useState('Daily');
  return wrap(
    <ScrollView contentContainerClassName="p-5 gap-4 items-center">
      <View className="w-full max-w-3xl gap-4">
        <View className="gap-1">
          <SectionOverline>Subscriptions</SectionOverline>
          <Text className="font-heading-bold text-3xl text-card-foreground">Choose your ritual</Text>
          <Text className="font-body text-sm text-muted-foreground">Cancel or pause anytime. First week free.</Text>
        </View>
        <Columns phone={1} tablet={3} desktop={3} gap={16}>
          {PLANS.map((p) => (
            <PlanCard key={p.name} plan={p} selected={sel === p.name} onPress={() => setSel(p.name)} />
          ))}
        </Columns>
        <WhatsAppCTAButton label={`Start ${sel} plan`} className="mt-2" />
      </View>
    </ScrollView>,
  );
}

// ── Menu (juice cards) ──
export function DelightsMenuScreen() {
  const juices = [
    { name: 'Green Vitality', accent: 'vitalityGreen' as const, volumeMl: 300, price: 180, image: 'https://images.unsplash.com/photo-1610970881699-44a5587cabec?w=600&q=85' },
    { name: 'Beet Revival', accent: 'beetRuby' as const, volumeMl: 300, price: 190, image: 'https://images.unsplash.com/photo-1622597467836-f3285f2131b8?w=600&q=85' },
    { name: 'Golden Balance', accent: 'turmericGold' as const, volumeMl: 250, price: 160, image: 'https://images.unsplash.com/photo-1615478503562-ec2d8aa0e24e?w=600&q=85' },
    { name: 'Citrus Morning', accent: 'citrusAmber' as const, volumeMl: 300, price: 170, image: 'https://images.unsplash.com/photo-1600271886742-f049cd451bba?w=600&q=85' },
  ];
  return wrap(
    <ScrollView contentContainerClassName="p-5 gap-4 items-center">
      <View className="w-full max-w-4xl gap-4">
        <View className="gap-1">
          <SectionOverline>The Menu</SectionOverline>
          <Text className="font-heading-bold text-3xl text-card-foreground">Today’s pressings</Text>
        </View>
        <AnchoredPriceDisplay retail={220} price={180} />
        <Columns phone={2} tablet={3} desktop={4} gap={16}>
          {juices.map((j) => (
            <JuiceCard key={j.name} juice={j} />
          ))}
        </Columns>
      </View>
    </ScrollView>,
  );
}

// ── Ingredient education ──
export function DelightsIngredientsScreen() {
  const ings = [
    { name: 'Spinach', oneLiner: 'Plant iron, folate & vitamin K.', image: 'https://images.unsplash.com/photo-1576045057995-568f588f82fb?w=500&q=85' },
    { name: 'Turmeric', oneLiner: 'Curcumin — anti-inflammatory root.', image: 'https://images.unsplash.com/photo-1615485500704-8e990f9900f7?w=500&q=85' },
    { name: 'Beetroot', oneLiner: 'Nitrates for blood flow & stamina.', image: 'https://images.unsplash.com/photo-1593105544559-ecb03bf76f1e?w=500&q=85' },
    { name: 'Amla', oneLiner: '20× the vitamin C of an orange.', image: 'https://images.unsplash.com/photo-1636201567770-96b3d0b40d9f?w=500&q=85' },
  ];
  return wrap(
    <ScrollView contentContainerClassName="p-5 gap-5 items-center">
      <View className="w-full max-w-4xl gap-5">
        <View className="gap-1">
          <SectionOverline>Know your ingredients</SectionOverline>
          <Text className="font-heading-bold text-3xl text-card-foreground">The Encyclopedia</Text>
        </View>
        <Columns phone={2} tablet={3} desktop={4} gap={12}>
          {ings.map((ing) => (
            <IngredientCard key={ing.name} {...ing} />
          ))}
        </Columns>
        <View className="items-center gap-3 pt-2">
          <SectionOverline>Collectible card — tap to flip</SectionOverline>
          <IngredientCardFlip
            name="Turmeric"
            hook="Paired with black pepper, absorption jumps 2,000%."
            cardNo={7}
            total={24}
            golden
            benefits={['Anti-inflammatory curcumin', 'Supports joint comfort', 'Aids digestion', 'Antioxidant-rich']}
            image="https://images.unsplash.com/photo-1615485500704-8e990f9900f7?w=500&q=85"
          />
        </View>
      </View>
    </ScrollView>,
  );
}

// ── 21-day tracker + streak ──
export function DelightsTrackerScreen() {
  const [checked, setChecked] = useState<boolean[]>(() => Array(21).fill(false).map((_, i) => i < 8));
  const toggle = (d: number) => setChecked((c) => c.map((v, i) => (i === d ? !v : v)));
  const streak = checked.filter(Boolean).length;
  return wrap(
    <ScrollView contentContainerClassName="p-5 gap-6 items-center">
      <View className="w-full max-w-2xl gap-6">
        <View className="gap-1">
          <SectionOverline>21-day reset</SectionOverline>
          <Text className="font-heading-bold text-3xl text-card-foreground">Your ritual</Text>
          <Text className="font-body text-sm text-muted-foreground">Tap each day you drink your juice. Small habits, compounding.</Text>
        </View>
        <StreakBadge days={streak} name="Jane’s streak" />
        <TwentyOneDayGrid checked={checked} onToggle={toggle} />
      </View>
    </ScrollView>,
  );
}

// ── Wellness quiz ──
export function DelightsQuizScreen() {
  return wrap(
    <View className="flex-1 items-center p-5">
      <View className="w-full max-w-3xl flex-1">
        <WellnessQuizStepper
          steps={[
            { key: 'goal', question: 'What’s your main wellness goal?', options: [
              { value: 'energy', label: 'More daily energy', icon: '⚡' },
              { value: 'gut', label: 'Better digestion', icon: '○' },
              { value: 'immunity', label: 'Stronger immunity', icon: '✦' },
              { value: 'skin', label: 'Glowing skin', icon: '☀' },
            ] },
            { key: 'taste', question: 'How do you like your juice?', options: [
              { value: 'green', label: 'Green & earthy' },
              { value: 'sweet', label: 'Fruity & sweet' },
              { value: 'tangy', label: 'Citrus & tangy' },
            ] },
            { key: 'time', question: 'When will you drink it?', options: [
              { value: 'am', label: 'First thing, empty stomach', icon: '○' },
              { value: 'mid', label: 'Mid-morning' },
              { value: 'pm', label: 'Evening reset' },
            ] },
          ]}
          onComplete={() => {}}
        />
      </View>
    </View>,
  );
}
