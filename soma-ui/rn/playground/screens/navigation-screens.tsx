import { ComponentPage, Demo } from '../component-page';
import { Tabs, Text } from '@/lib/components';

export function TabsScreen() {
  return (
    <ComponentPage title="Tabs" description="Tab bar with an animated sliding indicator.">
      <Demo label="Default">
        <Tabs
          className="w-full"
          items={[
            { value: 'account', label: 'Account', content: <Text variant="p">Account settings panel.</Text> },
            { value: 'password', label: 'Password', content: <Text variant="p">Password panel.</Text> },
            { value: 'team', label: 'Team', content: <Text variant="p">Team members panel.</Text> },
          ]}
        />
      </Demo>
    </ComponentPage>
  );
}
