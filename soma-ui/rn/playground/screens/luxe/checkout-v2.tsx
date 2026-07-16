import { View, ScrollView, TextInput, Pressable } from 'react-native';
import { MotiView } from 'moti';
import { Text } from '@/lib/components';

function Field({ label, placeholder, flex }: { label: string; placeholder: string; flex?: boolean }) {
  return (
    <View className={flex ? 'flex-1' : undefined}>
      <Text className="text-[10px] uppercase tracking-[2px] text-muted-foreground">{label}</Text>
      <TextInput
        placeholder={placeholder}
        placeholderTextColor="hsl(var(--muted-foreground))"
        className="border-b border-border py-2 font-body text-base text-foreground"
      />
    </View>
  );
}

/** V2 — Express checkout. Underlined fields, express-pay, refined density. */
export function LuxeCheckoutV2() {
  return (
    <View className="flex-1 items-center bg-background">
      <View className="w-full max-w-3xl flex-1">
        <ScrollView contentContainerClassName="px-6 pb-28 pt-14 gap-7" showsVerticalScrollIndicator={false}>
          <View className="flex-row items-center justify-between">
            <Text className="text-xl text-foreground">‹</Text>
            <Text className="text-xs uppercase tracking-[3px] text-muted-foreground">Checkout</Text>
            <View className="w-4" />
          </View>

          {/* Express pay */}
          <MotiView from={{ opacity: 0, translateY: 12 }} animate={{ opacity: 1, translateY: 0 }} transition={{ type: 'timing', duration: 400 }} className="gap-2">
            <Pressable className="items-center justify-center rounded-lg bg-foreground py-3.5 active:opacity-80">
              <Text className="font-heading-semibold text-sm text-background"> Pay</Text>
            </Pressable>
            <View className="flex-row items-center gap-3 py-1">
              <View className="h-px flex-1 bg-border" />
              <Text className="text-[10px] uppercase tracking-widest text-muted-foreground">or pay by card</Text>
              <View className="h-px flex-1 bg-border" />
            </View>
          </MotiView>

          <MotiView from={{ opacity: 0, translateY: 12 }} animate={{ opacity: 1, translateY: 0 }} transition={{ type: 'timing', duration: 400, delay: 100 }} className="gap-5">
            <Field label="Email" placeholder="jane@example.com" />
            <Field label="Card number" placeholder="1234 5678 9012 3456" />
            <View className="flex-row gap-6">
              <Field label="Expiry" placeholder="MM / YY" flex />
              <Field label="CVC" placeholder="•••" flex />
            </View>
            <Field label="Shipping address" placeholder="128 Birch Lane" />
          </MotiView>

          {/* Summary */}
          <MotiView from={{ opacity: 0 }} animate={{ opacity: 1 }} transition={{ type: 'timing', duration: 400, delay: 250 }} className="gap-2 rounded-xl bg-muted p-4">
            <SummaryRow k="Subtotal" v="$445" />
            <SummaryRow k="Shipping" v="$12" />
            <View className="mt-1 flex-row items-center justify-between border-t border-border pt-2">
              <Text className="font-body-semibold text-sm text-foreground">Total</Text>
              <Text className="font-heading-bold text-xl text-foreground">$457</Text>
            </View>
          </MotiView>
        </ScrollView>

        <View className="absolute inset-x-0 bottom-0 bg-background px-6 py-4">
          <Pressable className="items-center justify-center rounded-full bg-primary py-4 active:opacity-85">
            <Text className="text-sm tracking-[2px] text-primary-foreground">PAY $457</Text>
          </Pressable>
        </View>
      </View>
    </View>
  );
}

function SummaryRow({ k, v }: { k: string; v: string }) {
  return (
    <View className="flex-row items-center justify-between">
      <Text className="text-sm text-muted-foreground">{k}</Text>
      <Text className="text-sm text-foreground">{v}</Text>
    </View>
  );
}
