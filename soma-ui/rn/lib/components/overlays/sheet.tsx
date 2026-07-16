import { type ReactNode } from 'react';
import { Modal, Pressable, View, Text } from 'react-native';
import { Gesture, GestureDetector } from 'react-native-gesture-handler';
import Animated, {
  useSharedValue,
  useAnimatedStyle,
  withTiming,
  withSpring,
} from 'react-native-reanimated';
import { scheduleOnRN } from 'react-native-worklets';
import { vars } from 'nativewind';
import { useThemeVars } from '@/lib/theme/vars-context';
import { cn } from '@/lib/utils/cn';

export type SheetProps = {
  visible: boolean;
  onClose: () => void;
  title?: string;
  children?: ReactNode;
  className?: string;
};

/** Bottom sheet you can drag down to dismiss (or tap the backdrop). */
export function Sheet({ visible, onClose, title, children, className }: SheetProps) {
  // RN Modal renders outside the theme provider root View — re-apply active vars.
  // useThemeVars() reads ThemeVarsContext set by SomaThemeProvider or
  // SomaDelightsThemeProvider, so the correct palette is always used.
  const themeVars = vars(useThemeVars());

  const translateY = useSharedValue(0);

  const pan = Gesture.Pan()
    .onChange((e) => {
      translateY.value = Math.max(0, translateY.value + e.changeY);
    })
    .onEnd((e) => {
      // Drag past threshold or fling down → dismiss.
      if (translateY.value > 120 || e.velocityY > 800) {
        translateY.value = withTiming(600, { duration: 200 }, () => scheduleOnRN(onClose));
      } else {
        translateY.value = withSpring(0, { damping: 18, stiffness: 220 });
      }
    });

  const sheetStyle = useAnimatedStyle(() => ({ transform: [{ translateY: translateY.value }] }));

  const close = () => {
    translateY.value = 0;
    onClose();
  };

  return (
    <Modal visible={visible} transparent animationType="fade" onRequestClose={close}>
      <View style={themeVars} className="flex-1 justify-end">
        <Pressable className="absolute inset-0 bg-black/50" onPress={close} />
        <GestureDetector gesture={pan}>
          <Animated.View style={sheetStyle}>
            <View className={cn('rounded-t-2xl border-t border-border bg-card p-5 pb-8', className)}>
              <View className="mb-4 h-1 w-10 self-center rounded-full bg-border" />
              {title && (
                <Text className="mb-3 font-heading-semibold text-lg text-card-foreground">{title}</Text>
              )}
              {children}
            </View>
          </Animated.View>
        </GestureDetector>
      </View>
    </Modal>
  );
}
