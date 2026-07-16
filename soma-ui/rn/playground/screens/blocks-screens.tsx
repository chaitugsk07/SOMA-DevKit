import { ScrollView } from 'react-native';
import { Hero, Header, Footer, FeatureGrid, Faq, Login } from '@/lib/blocks';

const noop = () => {};

function BlockFrame({ children }: { children: React.ReactNode }) {
  return <ScrollView className="flex-1 bg-background">{children}</ScrollView>;
}

export function HeroScreen() {
  return (
    <BlockFrame>
      <Hero
        title="Build it once. Ship everywhere."
        subtitle="soma-ui brings the same design language to web, mobile, and native."
        cta={{ label: 'Get started', onPress: noop }}
        secondaryCta={{ label: 'Docs', onPress: noop }}
      />
    </BlockFrame>
  );
}

export function HeaderScreen() {
  return (
    <BlockFrame>
      <Header
        brand="soma-ui"
        actions={[
          { label: 'Docs', onPress: noop },
          { label: 'Sign in', variant: 'default', onPress: noop },
        ]}
      />
    </BlockFrame>
  );
}

export function FooterScreen() {
  return (
    <BlockFrame>
      <Footer
        brand="soma-ui"
        links={[
          { label: 'Docs', onPress: noop },
          { label: 'GitHub', onPress: noop },
          { label: 'Privacy', onPress: noop },
        ]}
        note="© 2026 soma. All rights reserved."
      />
    </BlockFrame>
  );
}

export function FeatureGridScreen() {
  return (
    <BlockFrame>
      <FeatureGrid
        heading="Why soma-ui"
        features={[
          { title: 'Cross-platform', description: 'One API across web, mobile, native.' },
          { title: 'Themeable', description: 'Light + dark via design tokens.' },
          { title: 'Animated', description: 'Smooth motion on the native thread.' },
          { title: 'You own it', description: 'Copy-paste source, no black box.' },
        ]}
      />
    </BlockFrame>
  );
}

export function FaqScreen() {
  return (
    <BlockFrame>
      <Faq
        items={[
          { question: 'Is it free?', answer: 'Yes — you own the source.' },
          { question: 'Does it support dark mode?', answer: 'Yes, via theme tokens.' },
          { question: 'Which platforms?', answer: 'Web, mobile, and native.' },
        ]}
      />
    </BlockFrame>
  );
}

export function LoginScreen() {
  return (
    <BlockFrame>
      <Login onSubmit={noop} />
    </BlockFrame>
  );
}
