/**
 * Web stand-in for LocationPicker.
 * react-native-maps has no web support — importing it on web crashes the bundle.
 * Metro resolves .web.tsx over .tsx for web builds automatically.
 */
import React from 'react';
import { View, TouchableOpacity, StyleSheet } from 'react-native';
import { MapPin, LocateFixed } from 'lucide-react-native';
import { Text } from '@/lib/components/data-display/text';
import { MapPlaceholder } from './map-placeholder';
import { Spinner } from '@/lib/components/feedback/spinner';
import { useThemeVars, hslFromVar } from '@/lib/theme/vars-context';
import { cn } from '@/lib/utils/cn';

// Re-export the same types so importers see a consistent shape on all platforms.
export type LocationPickerProps = {
  region: { latitude: number; longitude: number };
  onRegionSettled: (r: { latitude: number; longitude: number }) => void;
  onLocatePress?: () => void;
  locating?: boolean;
  height?: number;
  className?: string;
};

export function LocationPicker({
  onLocatePress,
  locating = false,
  height = 280,
  className,
}: LocationPickerProps): React.ReactElement {
  const vars = useThemeVars();

  return (
    <View
      className={cn('overflow-hidden rounded-xl border border-border bg-secondary', className)}
      style={{ height }}
    >
      {/* Decorative map background using the kit's SVG placeholder */}
      <MapPlaceholder className="absolute inset-0 h-full w-full opacity-40 rounded-none border-0" />

      {/* Centered message overlay */}
      <View style={StyleSheet.absoluteFill} className="items-center justify-center gap-2">
        <MapPin size={32} color={hslFromVar(vars['--muted-foreground'])} />
        <Text className="text-sm text-muted-foreground text-center px-4">
          Map picker is available in the mobile app
        </Text>
      </View>

      {/* Locate-me FAB — browser geolocation works on web */}
      {onLocatePress && (
        <TouchableOpacity
          onPress={onLocatePress}
          style={styles.fab}
          className="bg-card border border-border rounded-full shadow"
          accessibilityLabel="Locate me"
          accessibilityRole="button"
        >
          {locating ? (
            <Spinner size="small" />
          ) : (
            <LocateFixed size={20} color={hslFromVar(vars['--accent-foreground'])} />
          )}
        </TouchableOpacity>
      )}
    </View>
  );
}

const styles = StyleSheet.create({
  fab: {
    position: 'absolute',
    bottom: 12,
    right: 12,
    width: 44,
    height: 44,
    alignItems: 'center',
    justifyContent: 'center',
  },
});
