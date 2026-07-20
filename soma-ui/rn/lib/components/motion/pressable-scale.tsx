import { useState } from 'react';
import { Pressable, type PressableProps } from 'react-native';
import { MotiView } from 'moti';
import { useReducedMotion } from '@/lib/hooks';

export type PressableScaleProps = PressableProps & {
  /** Scale applied while pressed. */
  activeScale?: number;
  className?: string;
};

/** Pressable that springs down on press — the tactile-feel wrapper. */
export function PressableScale({
  activeScale = 0.96,
  className,
  onPressIn,
  onPressOut,
  children,
  ...props
}: PressableScaleProps) {
  const [pressed, setPressed] = useState(false);
  const reducedMotion = useReducedMotion();

  return (
    <Pressable
      onPressIn={(e) => {
        setPressed(true);
        onPressIn?.(e);
      }}
      onPressOut={(e) => {
        setPressed(false);
        onPressOut?.(e);
      }}
      {...props}
    >
      <MotiView
        animate={{ scale: !reducedMotion && pressed ? activeScale : 1 }}
        transition={reducedMotion
          ? { type: 'timing', duration: 0 }
          : { type: 'spring', damping: 15, stiffness: 250 }}
        className={className}
      >
        {children as React.ReactNode}
      </MotiView>
    </Pressable>
  );
}
