import React, { createContext, useContext } from 'react';
import { Pressable, View, Text } from 'react-native';
import { cn } from '@/lib/utils/cn';

type RadioGroupContextValue = {
  value: string;
  onChange: (value: string) => void;
};

const RadioGroupContext = createContext<RadioGroupContextValue | null>(null);

function useRadioGroup(): RadioGroupContextValue {
  const ctx = useContext(RadioGroupContext);
  if (ctx == null) throw new Error('RadioOption must be used inside a RadioGroup');
  return ctx;
}

export type RadioGroupProps = {
  value: string;
  onChange: (value: string) => void;
  children: React.ReactNode;
  className?: string;
  accessibilityLabel?: string;
};

export function RadioGroup({ value, onChange, children, className, accessibilityLabel }: RadioGroupProps) {
  return (
    <RadioGroupContext.Provider value={{ value, onChange }}>
      <View
        className={cn('flex-row flex-wrap gap-2', className)}
        accessibilityRole="radiogroup"
        accessibilityLabel={accessibilityLabel}
      >
        {children}
      </View>
    </RadioGroupContext.Provider>
  );
}

export type RadioOptionProps = {
  value: string;
  label: string;
};

export function RadioOption({ value, label }: RadioOptionProps) {
  const { value: selectedValue, onChange } = useRadioGroup();
  const selected = selectedValue === value;

  return (
    <Pressable
      onPress={() => onChange(value)}
      accessibilityRole="radio"
      accessibilityState={{ checked: selected }}
      accessibilityLabel={label}
      className={cn(
        'flex-row items-center justify-center rounded-full border px-4 active:opacity-70',
        selected ? 'border-accent-foreground bg-accent' : 'border-border bg-transparent',
      )}
      style={{ minHeight: 44 }}
    >
      <Text
        className={cn(
          'font-body text-sm',
          selected ? 'text-accent-foreground font-body-medium' : 'text-foreground',
        )}
      >
        {label}
      </Text>
    </Pressable>
  );
}
