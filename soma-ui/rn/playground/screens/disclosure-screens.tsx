import { View } from 'react-native';
import { ComponentPage, Demo } from '../component-page';
import { Accordion, Collapsible, Text } from '@/lib/components';

export function AccordionScreen() {
  return (
    <ComponentPage title="Accordion" description="Stacked disclosure rows. Single-open, animated height.">
      <Demo label="Default">
        <Accordion
          className="w-full"
          items={[
            { value: 'a', title: 'What is soma-ui?', content: <Text variant="muted" className="text-sm">A cross-platform design system.</Text> },
            { value: 'b', title: 'Is it themeable?', content: <Text variant="muted" className="text-sm">Yes — light and dark via tokens.</Text> },
            { value: 'c', title: 'Does it animate?', content: <Text variant="muted" className="text-sm">Yes — height transitions on open/close.</Text> },
          ]}
        />
      </Demo>
    </ComponentPage>
  );
}

export function CollapsibleScreen() {
  return (
    <ComponentPage title="Collapsible" description="Single expand/collapse section.">
      <Demo label="Default">
        <View className="w-full">
          <Collapsible title="Show details">
            <Text variant="muted" className="text-sm">
              Hidden content revealed with an animated height transition.
            </Text>
          </Collapsible>
        </View>
      </Demo>
    </ComponentPage>
  );
}
