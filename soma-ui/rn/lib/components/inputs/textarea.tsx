import { useState } from 'react';
import { TextInput, type TextInputProps } from 'react-native';
import { cn } from '@/lib/utils/cn';
import { hslFromVar, useThemeVars } from '@/lib/theme';

export type TextareaProps = TextInputProps & { className?: string };

export function Textarea({ className, onFocus, onBlur, editable, ...props }: TextareaProps) {
  const [focused, setFocused] = useState(false);
  const vars = useThemeVars();
  return (
    <TextInput
      multiline
      textAlignVertical="top"
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
        'min-h-[80px] rounded-lg border bg-card p-3 font-body text-sm text-foreground outline-none',
        focused ? 'border-ring' : 'border-input',
        editable === false && 'opacity-50',
        className,
      )}
      {...props}
    />
  );
}
