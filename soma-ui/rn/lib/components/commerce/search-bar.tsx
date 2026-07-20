import { useState } from 'react';
import { View, TextInput, Pressable, type TextInputProps } from 'react-native';
import { Text } from '@/lib/components/data-display/text';
import { cn } from '@/lib/utils/cn';
import { hslFromVar, useThemeVars } from '@/lib/theme';

export type SearchBarProps = Omit<TextInputProps, 'className'> & {
  onClear?: () => void;
  className?: string;
};

/** Search input with leading glyph, focus ring, and a clear affordance. */
export function SearchBar({ value, onClear, onFocus, onBlur, className, ...props }: SearchBarProps) {
  const [focused, setFocused] = useState(false);
  const vars = useThemeVars();
  return (
    <View
      className={cn(
        'h-11 flex-row items-center gap-2 rounded-lg border bg-card px-3',
        focused ? 'border-ring' : 'border-input',
        className,
      )}
    >
      <Text className="text-base text-muted-foreground">⌕</Text>
      <TextInput
        value={value}
        placeholder="Search products"
        placeholderTextColor={hslFromVar(vars['--muted-foreground'])}
        style={{ color: hslFromVar(vars['--foreground']) }}
        onFocus={(e) => { setFocused(true); onFocus?.(e); }}
        onBlur={(e) => { setFocused(false); onBlur?.(e); }}
        className="flex-1 font-body text-sm text-foreground"
        {...props}
      />
      {!!value && onClear && (
        <Pressable onPress={onClear} accessibilityLabel="Clear search" className="active:opacity-60">
          <Text className="text-sm text-muted-foreground">✕</Text>
        </Pressable>
      )}
    </View>
  );
}
