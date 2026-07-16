import { useState } from 'react';
import { View, Pressable } from 'react-native';
import { MotiView } from 'moti';
import { Text } from '@/lib/components/data-display/text';
import { SectionOverline } from './atoms';
import { cn } from '@/lib/utils/cn';

// ─── TwentyOneDayGrid (a.k.a HabitTrackerGrid) ───────────────────────────────
/** 21-day 7×3 check-off grid, one row per week. Tap toggles a day. */
export function TwentyOneDayGrid({ checked, onToggle, className }: {
  checked: boolean[]; // length 21
  onToggle: (day: number) => void;
  className?: string;
}) {
  return (
    <View className={cn('gap-3', className)}>
      {[0, 1, 2].map((week) => (
        <View key={week} className="gap-1.5">
          <Text className="font-body text-xs uppercase tracking-widest text-muted-foreground">Week {week + 1}</Text>
          <View className="flex-row justify-between">
            {Array.from({ length: 7 }).map((_, d) => {
              const day = week * 7 + d;
              const on = checked[day];
              return (
                <Pressable
                  key={day}
                  onPress={() => onToggle(day)}
                  accessibilityRole="checkbox"
                  accessibilityState={{ checked: on }}
                  className="items-center gap-1"
                >
                  <MotiView
                    animate={{ scale: on ? 1 : 0.96 }}
                    transition={{ type: 'spring', damping: 15, stiffness: 300 }}
                    className={cn('h-9 w-9 items-center justify-center rounded-lg border', on ? 'border-primary bg-primary' : 'border-border bg-card')}
                  >
                    <Text className={cn('font-body-medium text-xs', on ? 'text-primary-foreground' : 'text-muted-foreground')}>
                      {day + 1}
                    </Text>
                  </MotiView>
                </Pressable>
              );
            })}
          </View>
        </View>
      ))}
    </View>
  );
}

// ─── StreakBadge ─────────────────────────────────────────────────────────────
const MILESTONES = [7, 14, 21, 30, 60, 90, 180, 365];

export function StreakBadge({ days, name, className }: { days: number; name?: string; className?: string }) {
  const milestone = [...MILESTONES].reverse().find((m) => days >= m) ?? 0;
  return (
    <View className={cn('items-center gap-2 rounded-2xl bg-secondary p-5', className)}>
      <MotiView from={{ scale: 0.7, opacity: 0 }} animate={{ scale: 1, opacity: 1 }} transition={{ type: 'spring', damping: 12, stiffness: 180 }}
        className="h-16 w-16 items-center justify-center rounded-full border-2 border-primary">
        <Text className="font-heading-bold text-xl text-card-foreground">{days}</Text>
      </MotiView>
      <Text className="font-heading-semibold text-base text-card-foreground">
        {milestone >= 7 ? `${milestone}-day streak` : 'Keep going'}
      </Text>
      {name && <Text className="font-body text-xs text-muted-foreground">{name}</Text>}
    </View>
  );
}

// ─── QuizOptionButton ────────────────────────────────────────────────────────
export function QuizOptionButton({ label, icon, selected, onPress, className }: {
  label: string; icon?: string; selected?: boolean; onPress?: () => void; className?: string;
}) {
  return (
    <Pressable onPress={onPress} accessibilityRole="button" accessibilityState={{ selected }}
      className={cn('flex-row items-center gap-3 rounded-xl border p-4', selected ? 'border-primary bg-secondary' : 'border-border bg-card active:bg-accent', className)}>
      {icon && <Text className="text-lg">{icon}</Text>}
      <Text className={cn('flex-1 font-body-medium text-sm', selected ? 'text-card-foreground' : 'text-card-foreground')}>{label}</Text>
      <View className={cn('h-5 w-5 items-center justify-center rounded-full border-2', selected ? 'border-primary bg-primary' : 'border-border')}>
        {selected && <Text className="text-[10px] leading-none text-primary-foreground">✓</Text>}
      </View>
    </Pressable>
  );
}

// ─── WellnessQuizStepper ─────────────────────────────────────────────────────
export type QuizStep = { key: string; question: string; options: { value: string; label: string; icon?: string }[]; multi?: boolean };

/** Paginated wellness quiz. Renders one question at a time with progress + nav. */
export function WellnessQuizStepper({ steps, onComplete, className }: {
  steps: QuizStep[];
  onComplete?: (answers: Record<string, string[]>) => void;
  className?: string;
}) {
  const [i, setI] = useState(0);
  const [answers, setAnswers] = useState<Record<string, string[]>>({});
  const step = steps[i];
  const chosen = answers[step.key] ?? [];

  const pick = (value: string) => {
    setAnswers((cur) => {
      const prev = cur[step.key] ?? [];
      const next = step.multi
        ? (prev.includes(value) ? prev.filter((v) => v !== value) : [...prev, value])
        : [value];
      return { ...cur, [step.key]: next };
    });
  };

  const last = i === steps.length - 1;
  const next = () => (last ? onComplete?.(answers) : setI((v) => v + 1));

  return (
    <View className={cn('flex-1 gap-5', className)}>
      {/* Progress */}
      <View className="flex-row gap-1">
        {steps.map((_, idx) => (
          <View key={idx} className={cn('h-1 flex-1 rounded-full', idx <= i ? 'bg-primary' : 'bg-muted')} />
        ))}
      </View>

      <MotiView key={step.key} from={{ opacity: 0, translateX: 20 }} animate={{ opacity: 1, translateX: 0 }} transition={{ type: 'timing', duration: 250 }} className="flex-1 gap-4">
        <View className="gap-1">
          <SectionOverline>{`Question ${i + 1} of ${steps.length}`}</SectionOverline>
          <Text className="font-heading-bold text-2xl text-card-foreground">{step.question}</Text>
        </View>
        <View className="gap-2">
          {step.options.map((o) => (
            <QuizOptionButton key={o.value} label={o.label} icon={o.icon} selected={chosen.includes(o.value)} onPress={() => pick(o.value)} />
          ))}
        </View>
      </MotiView>

      <View className="flex-row gap-3">
        {i > 0 && (
          <Pressable onPress={() => setI((v) => v - 1)} className="items-center justify-center rounded-full border border-border px-6 py-3.5 active:bg-accent">
            <Text className="font-body-medium text-sm text-card-foreground">Back</Text>
          </Pressable>
        )}
        <Pressable onPress={next} disabled={chosen.length === 0}
          className={cn('flex-1 items-center justify-center rounded-full bg-primary py-3.5', chosen.length === 0 && 'opacity-40')}>
          <Text className="font-body-medium text-sm text-primary-foreground">{last ? 'See my plan' : 'Continue'}</Text>
        </Pressable>
      </View>
    </View>
  );
}
