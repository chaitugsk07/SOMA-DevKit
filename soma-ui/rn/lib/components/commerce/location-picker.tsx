import React, { useRef, useEffect, useCallback, useState } from 'react';
import { View, TouchableOpacity, StyleSheet } from 'react-native';
import MapView from 'react-native-maps';
import type { Region } from 'react-native-maps';
import { MapPin, LocateFixed } from 'lucide-react-native';
import { MotiView } from 'moti';
import { Spinner } from '@/lib/components/feedback/spinner';
import { useThemeVars, hslFromVar } from '@/lib/theme/vars-context';
import { cn } from '@/lib/utils/cn';
import { useReducedMotion } from '@/lib/hooks';

export type LocationPickerProps = {
  /** Controlled center; parent updates this to recenter (component animates there). */
  region: { latitude: number; longitude: number };
  /** Fires ~600ms after the user stops panning. */
  onRegionSettled: (r: { latitude: number; longitude: number }) => void;
  /** When provided, renders a locate-me FAB bottom-right. */
  onLocatePress?: () => void;
  /** FAB shows a Spinner while true. */
  locating?: boolean;
  /** Map container height. Default 280. */
  height?: number;
  className?: string;
};

const DELTA = 0.008;

export function LocationPicker({
  region,
  onRegionSettled,
  onLocatePress,
  locating = false,
  height = 280,
  className,
}: LocationPickerProps): React.ReactElement {
  const vars = useThemeVars();
  const primaryColor = hslFromVar(vars['--primary']);
  const reducedMotion = useReducedMotion();

  const mapRef = useRef<MapView>(null);
  const debounceRef = useRef<ReturnType<typeof setTimeout> | null>(null);
  const [isPanning, setIsPanning] = useState(false);

  // Animate to new region when the controlled prop changes identity.
  useEffect(() => {
    mapRef.current?.animateToRegion(
      { latitude: region.latitude, longitude: region.longitude, latitudeDelta: DELTA, longitudeDelta: DELTA },
      reducedMotion ? 0 : 300,
    );
  }, [region.latitude, region.longitude, reducedMotion]);

  const handleRegionChange = useCallback(() => {
    setIsPanning(true);
  }, []);

  const handleRegionChangeComplete = useCallback(
    (r: Region) => {
      setIsPanning(false);
      if (debounceRef.current) clearTimeout(debounceRef.current);
      debounceRef.current = setTimeout(() => {
        onRegionSettled({ latitude: r.latitude, longitude: r.longitude });
      }, 600);
    },
    [onRegionSettled],
  );

  return (
    <View
      className={cn('overflow-hidden rounded-xl border border-border', className)}
      style={{ height }}
    >
      <MapView
        ref={mapRef}
        style={StyleSheet.absoluteFill}
        initialRegion={{
          latitude: region.latitude,
          longitude: region.longitude,
          latitudeDelta: DELTA,
          longitudeDelta: DELTA,
        }}
        onRegionChange={handleRegionChange}
        onRegionChangeComplete={handleRegionChangeComplete}
        showsUserLocation={false}
        showsMyLocationButton={false}
        toolbarEnabled={false}
        rotateEnabled={false}
        pitchEnabled={false}
      />

      {/* Fixed center pin — map pans underneath; pin tip marks the selected point. */}
      <View style={[styles.pinContainer, { pointerEvents: 'none' }]}>
        <MotiView
          animate={{ translateY: !reducedMotion && isPanning ? -6 : 0 }}
          transition={reducedMotion
            ? { type: 'timing', duration: 0 }
            : { type: 'spring', damping: 18, stiffness: 220 }}
          style={styles.pinWrapper}
        >
          {/* Shadow dot below pin tip */}
          <View style={styles.pinShadow} />
          {/* Translate up by half pin height so the tip (bottom point) marks center */}
          <View style={{ transform: [{ translateY: -18 }] }}>
            <MapPin size={36} color={primaryColor} fill={primaryColor} />
          </View>
        </MotiView>
      </View>

      {/* Locate-me FAB */}
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
  pinContainer: {
    ...StyleSheet.absoluteFill,
    alignItems: 'center',
    justifyContent: 'center',
  },
  pinWrapper: {
    alignItems: 'center',
  },
  pinShadow: {
    width: 8,
    height: 4,
    borderRadius: 4,
    backgroundColor: 'rgba(0,0,0,0.18)',
    position: 'absolute',
    bottom: -2,
  },
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
