import { useState } from 'react';
import { View, TextInput, Pressable, ScrollView } from 'react-native';
import { SomaDelightsThemeProvider } from '@/lib/theme';
import {
  Text,
  OtpInput,
  SocialProviderList,
  Separator,
  FadeIn,
  Reveal,
} from '@/lib/components';

/** The corrected s-delights auth: phone → OTP → optional social link. */
export function DelightsAuthScreen() {
  const [stage, setStage] = useState<'phone' | 'otp' | 'link'>('phone');
  const [phone, setPhone] = useState('');
  const [code, setCode] = useState('');

  return (
    <SomaDelightsThemeProvider>
      <ScrollView contentContainerClassName="flex-grow justify-center p-6">
        <FadeIn>
          <View className="mx-auto w-full max-w-sm gap-6">
            {/* Brand */}
            <View className="items-center gap-2">
              <View className="h-12 w-12 items-center justify-center rounded-2xl bg-primary">
                <Text className="font-serif text-xl text-primary-foreground">s</Text>
              </View>
              <Text className="font-heading-bold text-2xl text-card-foreground">
                {stage === 'phone' ? 'Welcome to Soma' : stage === 'otp' ? 'Verify your number' : 'Almost there'}
              </Text>
              <Text className="text-center font-body text-sm text-muted-foreground">
                {stage === 'phone' && 'Enter your WhatsApp number to begin.'}
                {stage === 'otp' && `We sent a 6-digit code to ${phone || 'your phone'}.`}
                {stage === 'link' && 'Connect Google or Apple for faster sign-in next time.'}
              </Text>
            </View>

            {stage === 'phone' && (
              <Reveal direction="up" delay={80} className="gap-3">
                <View className="flex-row items-center gap-2 rounded-lg border border-border bg-input px-3">
                  <Text className="font-body-medium text-sm text-muted-foreground">+91</Text>
                  <View className="h-5 w-px bg-border" />
                  <TextInput
                    value={phone}
                    onChangeText={setPhone}
                    keyboardType="phone-pad"
                    placeholder="98765 43210"
                    placeholderTextColor="hsl(var(--muted-foreground))"
                    className="h-11 flex-1 font-body text-base text-foreground"
                  />
                </View>
                <Text className="font-body text-xs text-muted-foreground">We'll send a one-time code to this number.</Text>
                <Pressable onPress={() => setStage('otp')} className="items-center rounded-full bg-primary py-4 active:opacity-85">
                  <Text className="font-body-medium text-sm text-primary-foreground">Send code</Text>
                </Pressable>
                <View className="flex-row items-center gap-3 py-1">
                  <Separator className="flex-1" />
                  <Text className="font-body text-xs text-muted-foreground">or</Text>
                  <Separator className="flex-1" />
                </View>
                <SocialProviderList providers={['apple', 'google']} />
              </Reveal>
            )}

            {stage === 'otp' && (
              <Reveal direction="up" delay={80} className="gap-4">
                <OtpInput value={code} onChange={setCode} onComplete={() => setStage('link')} />
                <View className="flex-row justify-center gap-1">
                  <Text className="font-body text-sm text-muted-foreground">Didn't get it?</Text>
                  <Text className="font-body-medium text-sm text-card-foreground">Resend</Text>
                </View>
                <Pressable onPress={() => setStage('link')} className="items-center rounded-full bg-primary py-4 active:opacity-85">
                  <Text className="font-body-medium text-sm text-primary-foreground">Verify</Text>
                </Pressable>
                <Pressable onPress={() => setStage('phone')} className="items-center py-1"><Text className="font-body text-sm text-muted-foreground">Change number</Text></Pressable>
              </Reveal>
            )}

            {stage === 'link' && (
              <Reveal direction="up" delay={80} className="gap-3">
                <SocialProviderList providers={['apple', 'google']} />
                <Pressable className="items-center py-2"><Text className="font-body-medium text-sm text-muted-foreground">Skip for now</Text></Pressable>
              </Reveal>
            )}
          </View>
        </FadeIn>
      </ScrollView>
    </SomaDelightsThemeProvider>
  );
}
