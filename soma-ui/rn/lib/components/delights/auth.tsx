import { useState } from 'react';
import { View, TextInput, Pressable, Modal, type TextInputProps } from 'react-native';
import { Text } from '@/lib/components/data-display/text';
import { cn } from '@/lib/utils/cn';

// ─── PasswordInput ───────────────────────────────────────────────────────────
export function PasswordInput({ className, ...props }: TextInputProps & { className?: string }) {
  const [hidden, setHidden] = useState(true);
  const [focused, setFocused] = useState(false);
  return (
    <View className={cn('h-11 flex-row items-center rounded-lg border bg-input px-3', focused ? 'border-ring' : 'border-border', className)}>
      <TextInput
        secureTextEntry={hidden}
        placeholderTextColor="hsl(var(--muted-foreground))"
        onFocus={() => setFocused(true)}
        onBlur={() => setFocused(false)}
        className="flex-1 font-body text-sm text-foreground"
        {...props}
      />
      <Pressable onPress={() => setHidden((h) => !h)} accessibilityLabel={hidden ? 'Show password' : 'Hide password'} className="pl-2 active:opacity-60">
        <Text className="text-sm text-muted-foreground">{hidden ? '☉' : '⊘'}</Text>
      </Pressable>
    </View>
  );
}

// ─── SocialButton ────────────────────────────────────────────────────────────
export type SocialProvider = 'apple' | 'google' | 'microsoft' | 'github';

const providerMeta: Record<SocialProvider, { label: string; glyph: string }> = {
  apple: { label: 'Continue with Apple', glyph: '' },
  google: { label: 'Continue with Google', glyph: 'G' },
  microsoft: { label: 'Continue with Microsoft', glyph: '⊞' },
  github: { label: 'Continue with GitHub', glyph: '⌥' },
};

export function SocialButton({ provider, onPress, className }: { provider: SocialProvider; onPress?: () => void; className?: string }) {
  const m = providerMeta[provider];
  return (
    <Pressable
      onPress={onPress}
      accessibilityRole="button"
      accessibilityLabel={m.label}
      className={cn('h-12 flex-row items-center justify-center gap-2.5 rounded-lg border border-border bg-card active:bg-accent', className)}
    >
      <Text className="text-base text-card-foreground">{m.glyph}</Text>
      <Text className="font-body-medium text-sm text-card-foreground">{m.label}</Text>
    </Pressable>
  );
}

// ─── SocialProviderList ──────────────────────────────────────────────────────
/** Stacked social buttons. Apple first (iOS guideline). */
export function SocialProviderList({ providers = ['apple', 'google'], onSelect, className }: {
  providers?: SocialProvider[];
  onSelect?: (p: SocialProvider) => void;
  className?: string;
}) {
  return (
    <View className={cn('gap-2', className)}>
      {providers.map((p) => (
        <SocialButton key={p} provider={p} onPress={() => onSelect?.(p)} />
      ))}
    </View>
  );
}

// ─── AccountLinkingConfirmDialog ─────────────────────────────────────────────
/**
 * soma-iam policy I-1 / RFC 9700: when a social provider asserts email_verified
 * and the email matches an existing account, the owner MUST explicitly confirm
 * before merge. Silent merge is forbidden — this dialog is non-optional.
 */
export function AccountLinkingConfirmDialog({ visible, provider, email, onConfirm, onKeepSeparate }: {
  visible: boolean;
  provider: SocialProvider;
  email: string;
  onConfirm: () => void;
  onKeepSeparate: () => void;
}) {
  const name = provider.charAt(0).toUpperCase() + provider.slice(1);
  return (
    <Modal visible={visible} transparent animationType="fade" onRequestClose={onKeepSeparate}>
      <View className="flex-1 items-center justify-center bg-black/40 p-6">
        <View className="w-full max-w-sm gap-4 rounded-2xl bg-card p-5">
          <Text className="font-heading-semibold text-lg text-card-foreground">Link accounts?</Text>
          <Text className="font-body text-sm text-muted-foreground">
            Your {name} account ({email}) matches an existing Soma account. Link them so you can sign
            in with either? We'll email you to confirm before merging.
          </Text>
          <View className="gap-2">
            <Pressable onPress={onConfirm} className="items-center rounded-lg bg-primary py-3 active:opacity-85">
              <Text className="font-body-medium text-sm text-primary-foreground">Yes, link accounts</Text>
            </Pressable>
            <Pressable onPress={onKeepSeparate} className="items-center rounded-lg border border-border py-3 active:bg-accent">
              <Text className="font-body-medium text-sm text-card-foreground">No, keep separate</Text>
            </Pressable>
          </View>
        </View>
      </View>
    </Modal>
  );
}
