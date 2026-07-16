import { View } from 'react-native';

export type StepDotsProps = { current: number; total: number };

/** Segmented step progress bar (filled = completed / active). */
export function StepDots({ current, total }: StepDotsProps) {
  return (
    <View
      className="flex-row gap-1.5"
      accessibilityRole="progressbar"
      accessibilityLabel={`Step ${current} of ${total}`}
    >
      {Array.from({ length: total }).map((_, i) => (
        <View
          key={i}
          className={`h-1 flex-1 rounded-full ${i < current ? 'bg-primary' : 'bg-muted'}`}
        />
      ))}
    </View>
  );
}
