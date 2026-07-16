import { View } from 'react-native';
import { ComponentPage, Demo } from '../component-page';
import { Separator, Text } from '@/lib/components';

export function SeparatorScreen() {
  return (
    <ComponentPage title="Separator" description="Thin divider between content.">
      <Demo label="Horizontal">
        <View className="w-full gap-3">
          <Text variant="p">Above</Text>
          <Separator />
          <Text variant="p">Below</Text>
        </View>
      </Demo>
      <Demo label="Vertical">
        <View className="h-8 flex-row items-center gap-3">
          <Text variant="p">Left</Text>
          <Separator orientation="vertical" />
          <Text variant="p">Right</Text>
        </View>
      </Demo>
    </ComponentPage>
  );
}
