import { useState } from 'react';
import { View } from 'react-native';
import { BottomTabBar, PageTransition } from '@/lib/components';
import { StorefrontScreen } from './storefront-screen';
import { CartScreen } from './cart-screen';
import { TrackingScreen } from './tracking-screen';
import { AccountScreen } from './account-screen';

const TABS = [
  { key: 'shop', label: 'Shop', icon: '⌂' },
  { key: 'cart', label: 'Cart', icon: '⛨', badge: 3 },
  { key: 'orders', label: 'Orders', icon: '❏' },
  { key: 'account', label: 'Account', icon: '☺' },
] as const;

/** The customer app: a real bottom-tab shell tying the storefront screens together. */
export function ShopApp() {
  const [tab, setTab] = useState<string>('shop');

  return (
    <View className="flex-1 bg-background">
      <View className="flex-1">
        <PageTransition transitionKey={tab} variant="fade">
          {tab === 'shop' && <StorefrontScreen />}
          {tab === 'cart' && <CartScreen />}
          {tab === 'orders' && <TrackingScreen />}
          {tab === 'account' && <AccountScreen />}
        </PageTransition>
      </View>
      <BottomTabBar items={TABS as any} active={tab} onSelect={setTab} />
    </View>
  );
}
