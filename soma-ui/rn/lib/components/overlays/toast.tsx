import { createContext, useContext, useState, useCallback, type ReactNode } from 'react';
import { View } from 'react-native';
import { AnimatePresence, MotiView } from 'moti';
import { Text } from '@/lib/components/data-display/text';
import { cn } from '@/lib/utils/cn';

type ToastVariant = 'default' | 'destructive' | 'success';
type Toast = { id: number; title: string; description?: string; variant: ToastVariant };

type ToastContextValue = {
  toast: (t: { title: string; description?: string; variant?: ToastVariant }) => void;
};

const ToastContext = createContext<ToastContextValue | null>(null);

export function useToast(): ToastContextValue {
  const ctx = useContext(ToastContext);
  if (!ctx) throw new Error('useToast must be used within <ToastProvider>');
  return ctx;
}

const border: Record<ToastVariant, string> = {
  default: 'border-border',
  destructive: 'border-destructive',
  success: 'border-success',
};

let nextId = 0;

export function ToastProvider({ children, duration = 3000 }: { children: ReactNode; duration?: number }) {
  const [toasts, setToasts] = useState<Toast[]>([]);

  const toast = useCallback<ToastContextValue['toast']>(
    ({ title, description, variant = 'default' }) => {
      const id = nextId++;
      setToasts((cur) => [...cur, { id, title, description, variant }]);
      setTimeout(() => setToasts((cur) => cur.filter((t) => t.id !== id)), duration);
    },
    [duration],
  );

  return (
    <ToastContext.Provider value={{ toast }}>
      {children}
      <View pointerEvents="box-none" className="absolute inset-x-0 bottom-0 items-center gap-2 p-4">
        <AnimatePresence>
          {toasts.map((t) => (
            <MotiView
              key={t.id}
              pointerEvents="auto"
              from={{ opacity: 0, translateY: 20 }}
              animate={{ opacity: 1, translateY: 0 }}
              exit={{ opacity: 0, translateY: 20 }}
              transition={{ type: 'timing', duration: 200 }}
              className={cn('w-full max-w-md rounded-lg border bg-card p-4 shadow', border[t.variant])}
            >
              <Text className="font-heading-semibold text-sm text-card-foreground">{t.title}</Text>
              {t.description && (
                <Text className="mt-0.5 font-body text-xs text-muted-foreground">{t.description}</Text>
              )}
            </MotiView>
          ))}
        </AnimatePresence>
      </View>
    </ToastContext.Provider>
  );
}
