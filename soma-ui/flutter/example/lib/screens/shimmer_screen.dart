import 'package:flutter/material.dart';
import 'package:soma_ui/soma_ui.dart';
import '../component_page.dart';

class ShimmerScreen extends StatefulWidget {
  const ShimmerScreen({super.key});

  @override
  State<ShimmerScreen> createState() => _ShimmerScreenState();
}

class _ShimmerScreenState extends State<ShimmerScreen> {
  double _borderRadius = 6;

  @override
  Widget build(BuildContext context) {
    final c = SomaTheme.of(context);
    return ComponentPage(
      title: 'Shimmer',
      subtitle: 'Animated gradient sweep for skeleton loading placeholders.',
      preview: Column(
        mainAxisSize: MainAxisSize.min,
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          // Avatar circle + text lines
          Row(
            crossAxisAlignment: CrossAxisAlignment.start,
            children: [
              SomaShimmer(width: 48, height: 48, borderRadius: 999),
              const SizedBox(width: 12),
              Expanded(
                child: Column(
                  crossAxisAlignment: CrossAxisAlignment.start,
                  children: [
                    SomaShimmer(
                      height: 14,
                      borderRadius: _borderRadius,
                    ),
                    const SizedBox(height: 8),
                    SomaShimmer(
                      width: 160,
                      height: 12,
                      borderRadius: _borderRadius,
                    ),
                  ],
                ),
              ),
            ],
          ),
          const SizedBox(height: 16),
          SomaShimmer(height: 12, borderRadius: _borderRadius),
          const SizedBox(height: 8),
          SomaShimmer(height: 12, borderRadius: _borderRadius),
          const SizedBox(height: 8),
          SomaShimmer(width: 200, height: 12, borderRadius: _borderRadius),
        ],
      ),
      controls: Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          Row(children: [
            SizedBox(
              width: 120,
              child: Text(
                'Border radius',
                style: TextStyle(
                  color: c.mutedForeground,
                  fontFamily: 'Outfit',
                  fontSize: 13,
                ),
              ),
            ),
            Expanded(
              child: SomaSlider(
                value: _borderRadius,
                min: 0,
                max: 24,
                onChanged: (v) => setState(() => _borderRadius = v),
              ),
            ),
          ]),
        ],
      ),
    );
  }
}
