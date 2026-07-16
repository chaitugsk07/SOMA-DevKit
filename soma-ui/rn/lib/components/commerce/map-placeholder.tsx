import { View } from 'react-native';
import Svg, { Path, Circle, Line } from 'react-native-svg';
import { Text } from '@/lib/components/data-display/text';
import { cn } from '@/lib/utils/cn';
import { useThemeVars, hslFromVar } from '@/lib/theme/vars-context';

export type MapPlaceholderProps = {
  /** Optional label overlaid (e.g. an ETA). */
  eta?: string;
  className?: string;
};

// ponytail: stylized map (SVG grid + route), not a real map SDK. Swap in
// react-native-maps when a live map is actually needed — API stays the same.
export function MapPlaceholder({ eta, className }: MapPlaceholderProps) {
  const activeVars = useThemeVars();
  // SVG stroke/fill take concrete colours, not CSS vars — resolve from context.
  const gridColor = hslFromVar(activeVars['--border']);
  const routeColor = hslFromVar(activeVars['--primary']);
  const destColor = hslFromVar(activeVars['--success']);

  return (
    <View className={cn('overflow-hidden rounded-xl border border-border bg-muted', className)}>
      <Svg width="100%" height="100%" viewBox="0 0 300 180">
        {/* faint street grid */}
        {[40, 90, 140, 190, 240].map((x) => (
          <Line key={`v${x}`} x1={x} y1={0} x2={x} y2={180} stroke={gridColor} strokeWidth={1} />
        ))}
        {[45, 90, 135].map((y) => (
          <Line key={`h${y}`} x1={0} y1={y} x2={300} y2={y} stroke={gridColor} strokeWidth={1} />
        ))}
        {/* route */}
        <Path d="M40 150 L40 90 L140 90 L140 45 L240 45" stroke={routeColor} strokeWidth={4} fill="none" strokeLinecap="round" strokeLinejoin="round" />
        {/* origin + destination */}
        <Circle cx={40} cy={150} r={7} fill={routeColor} />
        <Circle cx={240} cy={45} r={7} fill={destColor} />
      </Svg>
      {eta && (
        <View className="absolute bottom-2 left-2 rounded-md bg-card px-2 py-1">
          <Text className="font-body-medium text-xs text-foreground">{eta}</Text>
        </View>
      )}
    </View>
  );
}
