import { type ReactNode } from 'react';
import { Modal, Pressable, View, Text } from 'react-native';
import { vars } from 'nativewind';
import { cn } from '@/lib/utils/cn';
import { useThemeVars } from '@/lib/theme/vars-context';

export type DialogProps = {
  visible: boolean;
  onClose: () => void;
  title?: string;
  description?: string;
  children?: ReactNode;
  className?: string;
};

export function Dialog({
  visible,
  onClose,
  title,
  description,
  children,
  className,
}: DialogProps) {
  // RN Modal renders in a separate native hierarchy outside the theme provider
  // root View — re-apply the active theme vars so tokens resolve correctly.
  // useThemeVars() reads ThemeVarsContext, which both SomaThemeProvider and
  // SomaDelightsThemeProvider populate, so the correct palette is always used.
  const themeVars = vars(useThemeVars());

  return (
    <Modal visible={visible} transparent animationType="fade" onRequestClose={onClose}>
      <View style={themeVars} className="flex-1">
        <Pressable className="flex-1 items-center justify-center bg-black/50 p-6" onPress={onClose}>
          <Pressable
            className={cn('w-full max-w-md rounded-lg border border-border bg-card p-5', className)}
            onPress={(e) => e.stopPropagation()}
          >
            {title && (
              <Text className="font-heading-semibold text-lg text-card-foreground">{title}</Text>
            )}
            {description && (
              <Text className="mt-1 font-body text-sm text-muted-foreground">{description}</Text>
            )}
            {children && <View className="mt-4">{children}</View>}
          </Pressable>
        </Pressable>
      </View>
    </Modal>
  );
}
