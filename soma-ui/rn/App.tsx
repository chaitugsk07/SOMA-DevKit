import './global.css';
import { useState } from 'react';
import { View, Pressable, Modal, useWindowDimensions } from 'react-native';
import { SafeAreaProvider, SafeAreaView } from 'react-native-safe-area-context';
import { GestureHandlerRootView } from 'react-native-gesture-handler';
import { StatusBar } from 'expo-status-bar';
import { useFonts } from 'expo-font';
import { vars } from 'nativewind';
import { SomaThemeProvider, useTheme, SomaThemeToggle, fontMap, lightVars, darkVars } from '@/lib/theme';
import { Text, ToastProvider, PageTransition } from '@/lib/components';
import { REGISTRY } from '@/playground/registry';
import { Home } from '@/playground/home';
import { SidebarContent } from '@/playground/sidebar';

const SCREENS = Object.fromEntries(
  REGISTRY.flatMap((c) => c.entries).map((e) => [e.name, e.screen]),
);

const MD = 768; // web md: breakpoint — persistent sidebar at/above this width

function Gallery() {
  const { scheme } = useTheme();
  const { width } = useWindowDimensions();
  const wide = width >= MD;
  const [active, setActive] = useState<string | null>(null); // null/'' = Home
  const [drawerOpen, setDrawerOpen] = useState(false);

  const Screen = active ? SCREENS[active] : null;
  const select = (name: string) => {
    setActive(name === '' ? null : name);
    setDrawerOpen(false);
  };

  const content = (
    <PageTransition transitionKey={active ?? 'home'} variant="slide">
      {Screen ? <Screen /> : <Home />}
    </PageTransition>
  );

  if (wide) {
    // Persistent sidebar + content (mirrors web flex row)
    return (
      <SafeAreaView className="flex-1 flex-row bg-background">
        <StatusBar style={scheme === 'dark' ? 'light' : 'dark'} />
        <View className="border-r border-border">
          <SidebarContent active={active} onSelect={select} />
        </View>
        <View className="flex-1">{content}</View>
      </SafeAreaView>
    );
  }

  // Narrow: fixed topbar + hamburger → slide-over drawer
  return (
    <SafeAreaView className="flex-1 bg-background">
      <StatusBar style={scheme === 'dark' ? 'light' : 'dark'} />
      <View className="flex-row items-center justify-between border-b border-border bg-card px-4 py-3">
        <Pressable
          onPress={() => setDrawerOpen(true)}
          accessibilityRole="button"
          accessibilityLabel="Open navigation"
          className="active:opacity-70"
        >
          <Text className="text-xl text-foreground">☰</Text>
        </Pressable>
        <Text className="font-heading-bold text-base text-foreground">
          {active ?? 'soma-ui'}
        </Text>
        <SomaThemeToggle />
      </View>
      <View className="flex-1">{content}</View>

      <Modal visible={drawerOpen} transparent animationType="fade" onRequestClose={() => setDrawerOpen(false)}>
        {/* Modal renders outside the provider's root View, so re-apply theme vars here. */}
        <View style={vars(scheme === 'light' ? lightVars : darkVars)} className="flex-1">
          <Pressable className="flex-1 flex-row bg-black/50" onPress={() => setDrawerOpen(false)}>
            <Pressable onPress={(e) => e.stopPropagation()}>
              <SidebarContent active={active} onSelect={select} />
            </Pressable>
          </Pressable>
        </View>
      </Modal>
    </SafeAreaView>
  );
}

export default function App() {
  const [fontsLoaded, error] = useFonts(fontMap);

  if (!fontsLoaded && !error) {
    return <View style={{ flex: 1 }} />;
  }

  return (
    <GestureHandlerRootView style={{ flex: 1 }}>
      <SafeAreaProvider>
        <SomaThemeProvider>
          <ToastProvider>
            <Gallery />
          </ToastProvider>
        </SomaThemeProvider>
      </SafeAreaProvider>
    </GestureHandlerRootView>
  );
}
