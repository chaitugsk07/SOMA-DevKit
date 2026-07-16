import { useState } from 'react';
import { View, ScrollView } from 'react-native';
import { Text, Input, Button, Separator, FadeIn, Reveal } from '@/lib/components';

/** Polished, minimalist auth — sign in / sign up toggle, social row, animated entrance. */
export function AuthScreen() {
  const [mode, setMode] = useState<'in' | 'up'>('in');
  const isUp = mode === 'up';

  return (
    <ScrollView className="flex-1 bg-background" contentContainerClassName="flex-grow justify-center p-6">
      <FadeIn>
        <View className="mx-auto w-full max-w-sm gap-6">
          {/* Brand mark */}
          <View className="items-center gap-2">
            <View className="h-12 w-12 items-center justify-center rounded-xl bg-primary">
              <Text className="font-heading-bold text-xl text-primary-foreground">S</Text>
            </View>
            <Text variant="h2" className="text-center">{isUp ? 'Create account' : 'Welcome back'}</Text>
            <Text variant="muted" className="text-center text-sm">
              {isUp ? 'Join soma to start shopping.' : 'Sign in to continue to soma.'}
            </Text>
          </View>

          {/* Form */}
          <Reveal direction="up" delay={80}>
            <View className="gap-3">
              {isUp && (
                <View className="gap-1.5">
                  <Text variant="small" className="text-muted-foreground">Name</Text>
                  <Input placeholder="Jane Doe" />
                </View>
              )}
              <View className="gap-1.5">
                <Text variant="small" className="text-muted-foreground">Email</Text>
                <Input placeholder="you@example.com" autoCapitalize="none" keyboardType="email-address" />
              </View>
              <View className="gap-1.5">
                <Text variant="small" className="text-muted-foreground">Password</Text>
                <Input placeholder="••••••••" secureTextEntry />
              </View>
              <Button className="mt-1 w-full" size="lg" label={isUp ? 'Create account' : 'Sign in'} />
            </View>
          </Reveal>

          {/* Divider */}
          <View className="flex-row items-center gap-3">
            <Separator className="flex-1" />
            <Text variant="small" className="text-muted-foreground">or</Text>
            <Separator className="flex-1" />
          </View>

          {/* Social row */}
          <View className="gap-2">
            <Button variant="outline" className="w-full" label="Continue with Apple" />
            <Button variant="outline" className="w-full" label="Continue with Google" />
          </View>

          {/* Toggle */}
          <View className="flex-row justify-center gap-1">
            <Text variant="muted" className="text-sm">{isUp ? 'Have an account?' : 'New here?'}</Text>
            <Text
              variant="p"
              className="text-sm text-primary"
              onPress={() => setMode(isUp ? 'in' : 'up')}
            >
              {isUp ? 'Sign in' : 'Create one'}
            </Text>
          </View>
        </View>
      </FadeIn>
    </ScrollView>
  );
}
