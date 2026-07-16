import { useState } from 'react';
import { ScrollView, View, Pressable } from 'react-native';
import { Text, SomaThemeToggle } from '@/lib';
import { cn } from '@/lib/utils/cn';
import { REGISTRY } from './registry';

const SIDEBAR_W = 240; // matches web w-60

/** One collapsible category group (mirrors web NavSection). */
function NavSection({
  category,
  entries,
  active,
  onSelect,
}: {
  category: string;
  entries: { name: string }[];
  active: string | null;
  onSelect: (name: string) => void;
}) {
  const [open, setOpen] = useState(true);
  return (
    <View className="gap-0.5">
      <Pressable
        onPress={() => setOpen((o) => !o)}
        className="flex-row items-center justify-between px-3 py-2"
      >
        <Text className="font-heading-semibold text-xs uppercase tracking-widest text-muted-foreground">
          {category}
        </Text>
        <Text className="text-xs text-muted-foreground">{open ? '▾' : '▸'}</Text>
      </Pressable>
      {open &&
        entries.map((entry) => {
          const isActive = active === entry.name;
          return (
            <Pressable
              key={entry.name}
              onPress={() => onSelect(entry.name)}
              accessibilityRole="button"
              accessibilityState={{ selected: isActive }}
              className={cn(
                'rounded-md px-3 py-2',
                isActive
                  ? 'border-l-2 border-primary bg-accent pl-[10px]'
                  : 'active:bg-accent',
              )}
            >
              <Text
                className={cn(
                  'text-sm',
                  isActive ? 'text-foreground' : 'text-muted-foreground',
                )}
              >
                {entry.name}
              </Text>
            </Pressable>
          );
        })}
    </View>
  );
}

/** Sidebar panel: brand header + theme toggle, then grouped nav. Shared by
 *  the persistent (wide) sidebar and the mobile drawer. */
export function SidebarContent({
  active,
  onSelect,
}: {
  active: string | null;
  onSelect: (name: string) => void;
}) {
  return (
    <View className="flex-1 bg-card" style={{ width: SIDEBAR_W }}>
      <View className="flex-row items-center justify-between border-b border-border p-4">
        <View>
          <Text className="font-heading-bold text-lg text-foreground">soma-ui</Text>
          <Text className="text-xs text-muted-foreground">React Native</Text>
        </View>
        <SomaThemeToggle />
      </View>
      <ScrollView contentContainerClassName="p-4 gap-6">
        <Pressable
          onPress={() => onSelect('')}
          className={cn(
            'rounded-md px-3 py-2',
            active === '' || active === null
              ? 'border-l-2 border-primary bg-accent pl-[10px]'
              : 'active:bg-accent',
          )}
        >
          <Text
            className={cn(
              'text-sm',
              active === '' || active === null ? 'text-foreground' : 'text-muted-foreground',
            )}
          >
            Home
          </Text>
        </Pressable>
        {REGISTRY.map((cat) => (
          <NavSection
            key={cat.category}
            category={cat.category}
            entries={cat.entries}
            active={active}
            onSelect={onSelect}
          />
        ))}
      </ScrollView>
    </View>
  );
}

export { SIDEBAR_W };
