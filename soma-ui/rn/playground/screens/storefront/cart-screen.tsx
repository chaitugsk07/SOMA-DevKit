import { useState } from 'react';
import { View, ScrollView } from 'react-native';
import {
  Text,
  Button,
  Separator,
  PriceTag,
  CartLineItem,
  AnimatedList,
  AnimatedListItem,
  FadeIn,
} from '@/lib/components';
import { PRODUCTS } from './data';

type Line = { product: (typeof PRODUCTS)[number]; qty: number };

/** Cart: animated line items, live-updating summary. */
export function CartScreen() {
  const [lines, setLines] = useState<Line[]>([
    { product: PRODUCTS[0], qty: 1 },
    { product: PRODUCTS[2], qty: 1 },
    { product: PRODUCTS[4], qty: 2 },
  ]);

  const setQty = (id: string, qty: number) =>
    setLines((cur) => cur.map((l) => (l.product.id === id ? { ...l, qty } : l)));
  const remove = (id: string) => setLines((cur) => cur.filter((l) => l.product.id !== id));

  const subtotal = lines.reduce((n, l) => n + l.product.price * l.qty, 0);
  const shipping = subtotal > 0 ? 9.0 : 0;
  const total = subtotal + shipping;

  return (
    <View className="flex-1 bg-background">
      <ScrollView contentContainerClassName="pb-40 items-center">
        <View className="w-full max-w-2xl p-4">
          <FadeIn>
            <Text variant="h1" className="mb-2">Cart</Text>
          </FadeIn>

          {lines.length === 0 ? (
            <View className="items-center py-16">
              <Text variant="muted">Your cart is empty.</Text>
            </View>
          ) : (
            <AnimatedList>
              {lines.map((l, i) => (
                <AnimatedListItem key={l.product.id}>
                  {i > 0 && <Separator />}
                  <CartLineItem
                    product={l.product}
                    quantity={l.qty}
                    onQuantityChange={(q) => setQty(l.product.id, q)}
                    onRemove={() => remove(l.product.id)}
                  />
                </AnimatedListItem>
              ))}
            </AnimatedList>
          )}
        </View>
      </ScrollView>

      {/* Sticky summary */}
      {lines.length > 0 && (
        <View className="absolute inset-x-0 bottom-0 gap-3 border-t border-border bg-card p-4">
          <View className="gap-1.5">
            <Row label="Subtotal" amount={subtotal} />
            <Row label="Shipping" amount={shipping} />
            <Separator className="my-1" />
            <View className="flex-row items-center justify-between">
              <Text variant="p" className="font-body-semibold">Total</Text>
              <PriceTag amount={total} size="md" />
            </View>
          </View>
          <Button size="lg" label="Checkout" />
        </View>
      )}
    </View>
  );
}

function Row({ label, amount }: { label: string; amount: number }) {
  return (
    <View className="flex-row items-center justify-between">
      <Text variant="muted" className="text-sm">{label}</Text>
      <Text variant="p" className="text-sm">${amount.toFixed(2)}</Text>
    </View>
  );
}
