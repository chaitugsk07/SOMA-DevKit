import { useState, forwardRef } from 'react';
import { TextInput, Text, View, type TextInputProps } from 'react-native';
import { cn } from '@/lib/utils/cn';
import { hslFromVar, useThemeVars } from '@/lib/theme';

export type InputProps = TextInputProps & {
  className?: string;
  label?: string;
  helperText?: string;
  error?: string;
};

export const Input = forwardRef<TextInput, InputProps>(function Input(
  { className, onFocus, onBlur, editable, label, helperText, error, ...props },
  ref,
) {
  const [focused, setFocused] = useState(false);
  const vars = useThemeVars();

  const textInput = (
    <TextInput
      ref={ref}
      editable={editable}
      placeholderTextColor={hslFromVar(vars['--muted-foreground'])}
      style={{ color: hslFromVar(vars['--foreground']) }}
      onFocus={(e) => {
        setFocused(true);
        onFocus?.(e);
      }}
      onBlur={(e) => {
        setFocused(false);
        onBlur?.(e);
      }}
      className={cn(
        'h-11 rounded-lg border bg-card px-3 font-body text-sm text-foreground outline-none',
        error != null ? 'border-destructive' : focused ? 'border-ring' : 'border-input',
        editable === false && 'opacity-50',
        className,
      )}
      {...props}
    />
  );

  // Bare usage: no new props → return TextInput directly, preserving all existing layout behaviour
  // (flex-1, width classes, etc. applied via className continue to target the TextInput as before).
  if (label == null && helperText == null && error == null) {
    return textInput;
  }

  return (
    <View>
      {label != null && (
        <Text className="mb-1.5 font-body-medium text-sm text-foreground">{label}</Text>
      )}
      {textInput}
      {(error != null || helperText != null) && (
        <Text className={cn('mt-1.5 text-xs', error != null ? 'text-destructive' : 'text-muted-foreground')}>
          {error ?? helperText}
        </Text>
      )}
    </View>
  );
});
