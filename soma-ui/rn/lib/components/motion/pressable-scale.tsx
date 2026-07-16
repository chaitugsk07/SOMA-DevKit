import { useState } from 'react';
import { Pressable, type PressableProps } from 'react-native';
import { MotiView } from 'moti';

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
        animate={{ scale: pressed ? activeScale : 1 }}
        transition={{ type: 'spring', damping: 15, stiffness: 250 }}
        className={className}
      >
        {children as React.ReactNode}
      </MotiView>
    </Pressable>
  );
}
