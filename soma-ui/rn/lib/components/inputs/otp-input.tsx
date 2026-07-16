import { useRef, useState } from 'react';
import { View, TextInput } from 'react-native';
import { cn } from '@/lib/utils/cn';

export type OtpInputProps = {
  length?: number;
  value: string;
  onChange: (code: string) => void;
  /** Fired when all digits are entered. */
  onComplete?: (code: string) => void;
  className?: string;
  /** When true, all boxes are non-editable and the container is dimmed. */
  disabled?: boolean;
  /** When true, the first box gets autoFocus. */
  autoFocus?: boolean;
};

/** N-box one-time-code entry: auto-advance, backspace-to-previous, paste support. */
export function OtpInput({ length = 6, value, onChange, onComplete, className, disabled, autoFocus }: OtpInputProps) {
  const refs = useRef<(TextInput | null)[]>([]);
  const [focused, setFocused] = useState<number | null>(null);
  const digits = value.padEnd(length, ' ').slice(0, length).split('');

  const setAt = (i: number, char: string) => {
    // Handle paste of a full code into one box
    const clean = char.replace(/\D/g, '');
    if (clean.length > 1) {
      const next = clean.slice(0, length);
      onChange(next);
      if (next.length === length) onComplete?.(next);
      // ponytail: defer focus so onChange state commits before the blur fires on web
      setTimeout(() => refs.current[Math.min(next.length, length - 1)]?.focus(), 0);
      return;
    }
    const arr = [...digits];
    arr[i] = clean || ' ';
    const joined = arr.join('').replace(/ /g, '');
    onChange(joined);
    // ponytail: defer focus so onChange state commits before the blur fires on web
    if (clean && i < length - 1) setTimeout(() => refs.current[i + 1]?.focus(), 0);
    if (joined.length === length) onComplete?.(joined);
  };

  return (
    <View className={cn('flex-row justify-center gap-2', className)} style={disabled ? { opacity: 0.5 } : undefined}>
      {Array.from({ length }).map((_, i) => (
        <TextInput
          key={i}
          ref={(r) => { refs.current[i] = r; }}
          value={digits[i].trim()}
          onChangeText={(t) => setAt(i, t)}
          onKeyPress={(e) => {
            if (e.nativeEvent.key === 'Backspace' && !digits[i].trim() && i > 0) {
              refs.current[i - 1]?.focus();
            }
          }}
          onFocus={() => setFocused(i)}
          onBlur={() => setFocused(null)}
          keyboardType="number-pad"
          maxLength={i === 0 ? length : 1}
          selectTextOnFocus
          editable={!disabled}
          autoFocus={i === 0 ? autoFocus : undefined}
          className={cn(
            'h-14 w-12 rounded-lg border bg-input text-center font-heading-semibold text-xl text-foreground outline-none',
            focused === i ? 'border-ring' : 'border-border',
          )}
        />
      ))}
    </View>
  );
}
