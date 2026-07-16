import { View } from 'react-native';
import { Text } from '@/lib/components';
import { Accordion } from '@/lib/components/disclosure';

export type FaqItem = { question: string; answer: string };

export type FaqProps = {
  heading?: string;
  items: FaqItem[];
};

/** FAQ section built on the Accordion (single-open). */
export function Faq({ heading = 'Frequently asked questions', items }: FaqProps) {
  return (
    <View className="gap-4 bg-background px-6 py-12">
      <Text variant="h2" className="text-center">{heading}</Text>
      <Accordion
        items={items.map((it, i) => ({
          value: String(i),
          title: it.question,
          content: <Text variant="muted" className="text-sm">{it.answer}</Text>,
        }))}
      />
    </View>
  );
}
