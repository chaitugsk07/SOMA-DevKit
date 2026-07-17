import { createContext, useContext, useState, useCallback, type ReactNode } from 'react';
import { View } from 'react-native';
import { AnimatePresence, MotiView } from 'moti';
import { Text } from '@/lib/components/data-display/text';
import { cn } from '@/lib/utils/cn';

type ToastVariant = 'default' | 'destructive' | 'success' | 'warning' | 'info';
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
  warning: 'border-warning',
  info: 'border-info',
};

const titleColor: Record<ToastVariant, string> = {
  default: 'text-card-foreground',
  destructive: 'text-destructive',
  success: 'text-success',
  warning: 'text-warning',
  info: 'text-info',
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
              className={cn('w-full max-w-md rounded-lg border border-l-4 bg-card p-4 shadow', border[t.variant])}
            >
              <Text className={cn('font-heading-semibold text-sm', titleColor[t.variant])}>{t.title}</Text>
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
