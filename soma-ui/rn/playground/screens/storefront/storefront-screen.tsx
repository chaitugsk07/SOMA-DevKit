import { useState } from 'react';
import { View, ScrollView, Pressable } from 'react-native';
import { Text, SearchBar, ProductCard, FadeIn, AnimateGroup } from '@/lib/components';
import { useBreakpoint } from '@/lib/hooks';
import { PRODUCTS } from './data';

const CATEGORIES = ['All', 'New', 'Apparel', 'Tech', 'Bags'];

/** Storefront: brand header, search, category chips, staggered product grid. */
export function StorefrontScreen() {
  const [query, setQuery] = useState('');
  const [cat, setCat] = useState('All');
  const items = PRODUCTS.filter((p) => p.name.toLowerCase().includes(query.toLowerCase()));
  const bp = useBreakpoint();
  const itemW = bp.isDesktop ? 'w-[23%]' : bp.isTablet ? 'w-[31%]' : 'w-[48%]';

  return (
    <ScrollView className="flex-1 bg-background" contentContainerClassName="p-4 gap-4 items-center">
      <View className="w-full max-w-5xl gap-4">
        {/* Header */}
        <FadeIn>
          <View className="flex-row items-end justify-between">
            <View>
              <Text variant="muted" className="text-sm">Good morning</Text>
              <Text variant="h1">Shop</Text>
            </View>
            <View className="h-10 w-10 items-center justify-center rounded-full bg-muted">
              <Text className="font-body-semibold text-sm text-foreground">JD</Text>
            </View>
          </View>
        </FadeIn>

        <SearchBar value={query} onChangeText={setQuery} onClear={() => setQuery('')} />

        {/* Category chips */}
        <ScrollView horizontal showsHorizontalScrollIndicator={false} contentContainerClassName="gap-2">
          {CATEGORIES.map((c) => {
            const active = c === cat;
            return (
              <Pressable
                key={c}
                onPress={() => setCat(c)}
                className={
                  'rounded-full border px-4 py-1.5 ' +
                  (active ? 'border-primary bg-primary' : 'border-border bg-card active:bg-accent')
                }
              >
                <Text className={'font-body-medium text-sm ' + (active ? 'text-primary-foreground' : 'text-foreground')}>
                  {c}
                </Text>
              </Pressable>
            );
          })}
        </ScrollView>

        {/* Product grid — staggered entrance, responsive columns */}
        <AnimateGroup className="flex-row flex-wrap justify-between" itemClassName={'mb-3 ' + itemW} stagger={70}>
          {items.map((p) => (
            <ProductCard key={p.id} product={p} />
          ))}
        </AnimateGroup>

        {items.length === 0 && (
          <View className="items-center py-12">
            <Text variant="muted">{'No products match "' + query + '".'}</Text>
          </View>
        )}
      </View>
    </ScrollView>
  );
}
