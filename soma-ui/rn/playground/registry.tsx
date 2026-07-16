import { type ComponentType } from 'react';
import {
  ButtonScreen,
  InputScreen,
  SwitchScreen,
  CheckboxScreen,
  TextareaScreen,
} from './screens/inputs-screens';
import {
  TextScreen,
  BadgeScreen,
  AvatarScreen,
  CardScreen,
} from './screens/data-display-screens';
import {
  AlertScreen,
  SkeletonScreen,
  SpinnerScreen,
  ProgressScreen,
} from './screens/feedback-screens';
import { SeparatorScreen } from './screens/layout-screens';
import { TabsScreen } from './screens/navigation-screens';
import { AccordionScreen, CollapsibleScreen } from './screens/disclosure-screens';
import {
  DialogScreen,
  ToastScreen,
  TooltipScreen,
  SheetScreen,
} from './screens/overlays-screens';
import {
  FadeInScreen,
  SlideInScreen,
  AnimateGroupScreen,
  PressableScaleScreen,
  ShimmerScreen,
} from './screens/motion-screens';
import {
  CountUpScreen,
  RevealScreen,
  AnimatedRingScreen,
  TypewriterScreen,
  AnimatedNumberScreen,
  PulseScreen,
  AnimatedListScreen,
} from './screens/motion2-screens';
import {
  HeroScreen,
  HeaderScreen,
  FooterScreen,
  FeatureGridScreen,
  FaqScreen,
  LoginScreen,
} from './screens/blocks-screens';
import { AuthScreen } from './screens/storefront/auth-screen';
import { StorefrontScreen } from './screens/storefront/storefront-screen';
import { ProductDetailScreen } from './screens/storefront/product-detail-screen';
import { CartScreen } from './screens/storefront/cart-screen';
import { CheckoutScreen } from './screens/storefront/checkout-screen';
import { TrackingScreen } from './screens/storefront/tracking-screen';
import { QueueScreen } from './screens/driver/queue-screen';
import { ActiveScreen } from './screens/driver/active-screen';
import { EarningsScreen } from './screens/driver/earnings-screen';
import { LuxeStorefrontV1 } from './screens/luxe/storefront-v1';
import { LuxeStorefrontV2 } from './screens/luxe/storefront-v2';
import { LuxeStorefrontV3 } from './screens/luxe/storefront-v3';
import { LuxeProductV1 } from './screens/luxe/product-v1';
import { LuxeProductV2 } from './screens/luxe/product-v2';
import { LuxeProductV3 } from './screens/luxe/product-v3';
import { LuxeCheckoutV1 } from './screens/luxe/checkout-v1';
import { LuxeCheckoutV2 } from './screens/luxe/checkout-v2';
import { LuxeCheckoutV3 } from './screens/luxe/checkout-v3';
import { DelightsAuthScreen } from './screens/delights/auth-screen';
import {
  DelightsSkuScreen,
  DelightsPlansScreen,
  DelightsMenuScreen,
  DelightsIngredientsScreen,
  DelightsTrackerScreen,
  DelightsQuizScreen,
} from './screens/delights/showcase-screens';
import { PhoneFrame } from './phone-frame';

export type Entry = { name: string; screen: ComponentType };
export type Category = { category: string; entries: Entry[] };

/** Wrap a full-bleed app screen so it reads as a phone frame on desktop. */
const phone = (Screen: ComponentType): ComponentType => {
  const Framed = () => (
    <PhoneFrame>
      <Screen />
    </PhoneFrame>
  );
  return Framed;
};

export const REGISTRY: Category[] = [
  {
    category: '🥤 s-delights',
    entries: [
      { name: 'Sign in · Phone/OTP', screen: phone(DelightsAuthScreen) },
      { name: 'Wellness Quiz', screen: phone(DelightsQuizScreen) },
      { name: 'SKU Bottle Page', screen: phone(DelightsSkuScreen) },
      { name: 'Subscription Plans', screen: phone(DelightsPlansScreen) },
      { name: 'The Menu', screen: phone(DelightsMenuScreen) },
      { name: 'Ingredient Encyclopedia', screen: phone(DelightsIngredientsScreen) },
      { name: '21-Day Tracker', screen: phone(DelightsTrackerScreen) },
    ],
  },
  {
    category: '✦ Luxe (redesign)',
    entries: [
      { name: 'Storefront · Carousel', screen: phone(LuxeStorefrontV1) },
      { name: 'Storefront · Index', screen: phone(LuxeStorefrontV2) },
      { name: 'Storefront · Gallery', screen: phone(LuxeStorefrontV3) },
      { name: 'Product · Immersive', screen: phone(LuxeProductV1) },
      { name: 'Product · Split', screen: phone(LuxeProductV2) },
      { name: 'Product · Minimal', screen: phone(LuxeProductV3) },
      { name: 'Order · Thank You', screen: phone(LuxeCheckoutV1) },
      { name: 'Checkout · Express', screen: phone(LuxeCheckoutV2) },
      { name: 'Bag · Editorial', screen: phone(LuxeCheckoutV3) },
    ],
  },
  {
    category: '🛍 Storefront',
    entries: [
      { name: 'Sign in / Sign up', screen: phone(AuthScreen) },
      { name: 'Storefront', screen: phone(StorefrontScreen) },
      { name: 'Product Detail', screen: phone(ProductDetailScreen) },
      { name: 'Cart', screen: phone(CartScreen) },
      { name: 'Checkout', screen: phone(CheckoutScreen) },
      { name: 'Order Tracking', screen: phone(TrackingScreen) },
    ],
  },
  {
    category: '🚗 Driver',
    entries: [
      { name: 'Delivery Queue', screen: phone(QueueScreen) },
      { name: 'Active Delivery', screen: phone(ActiveScreen) },
      { name: 'Earnings', screen: phone(EarningsScreen) },
    ],
  },
  {
    category: 'Inputs',
    entries: [
      { name: 'Button', screen: ButtonScreen },
      { name: 'Input', screen: InputScreen },
      { name: 'Switch', screen: SwitchScreen },
      { name: 'Checkbox', screen: CheckboxScreen },
      { name: 'Textarea', screen: TextareaScreen },
    ],
  },
  {
    category: 'Data Display',
    entries: [
      { name: 'Text', screen: TextScreen },
      { name: 'Badge', screen: BadgeScreen },
      { name: 'Avatar', screen: AvatarScreen },
      { name: 'Card', screen: CardScreen },
    ],
  },
  {
    category: 'Feedback',
    entries: [
      { name: 'Alert', screen: AlertScreen },
      { name: 'Skeleton', screen: SkeletonScreen },
      { name: 'Spinner', screen: SpinnerScreen },
      { name: 'Progress', screen: ProgressScreen },
    ],
  },
  {
    category: 'Navigation',
    entries: [{ name: 'Tabs', screen: TabsScreen }],
  },
  {
    category: 'Disclosure',
    entries: [
      { name: 'Accordion', screen: AccordionScreen },
      { name: 'Collapsible', screen: CollapsibleScreen },
    ],
  },
  {
    category: 'Overlays',
    entries: [
      { name: 'Dialog', screen: DialogScreen },
      { name: 'Sheet', screen: SheetScreen },
      { name: 'Toast', screen: ToastScreen },
      { name: 'Tooltip', screen: TooltipScreen },
    ],
  },
  {
    category: 'Motion',
    entries: [
      { name: 'FadeIn', screen: FadeInScreen },
      { name: 'SlideIn', screen: SlideInScreen },
      { name: 'Reveal', screen: RevealScreen },
      { name: 'AnimateGroup', screen: AnimateGroupScreen },
      { name: 'AnimatedList', screen: AnimatedListScreen },
      { name: 'PressableScale', screen: PressableScaleScreen },
      { name: 'Pulse', screen: PulseScreen },
      { name: 'Shimmer', screen: ShimmerScreen },
    ],
  },
  {
    category: 'Data Animation',
    entries: [
      { name: 'CountUp', screen: CountUpScreen },
      { name: 'AnimatedNumber', screen: AnimatedNumberScreen },
      { name: 'AnimatedRing', screen: AnimatedRingScreen },
      { name: 'Typewriter', screen: TypewriterScreen },
    ],
  },
  {
    category: 'Layout',
    entries: [{ name: 'Separator', screen: SeparatorScreen }],
  },
  {
    category: 'Blocks',
    entries: [
      { name: 'Hero', screen: HeroScreen },
      { name: 'Header', screen: HeaderScreen },
      { name: 'Footer', screen: FooterScreen },
      { name: 'Feature Grid', screen: FeatureGridScreen },
      { name: 'FAQ', screen: FaqScreen },
      { name: 'Login', screen: LoginScreen },
    ],
  },
];
