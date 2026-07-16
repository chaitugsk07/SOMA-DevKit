import 'package:flutter/material.dart';
import 'package:soma_ui/soma_ui.dart';
import '../component_page.dart';

class SkeletonScreen extends StatefulWidget {
  const SkeletonScreen({super.key});

  @override
  State<SkeletonScreen> createState() => _SkeletonScreenState();
}

class _SkeletonScreenState extends State<SkeletonScreen> {
  bool _loaded = false;

  @override
  Widget build(BuildContext context) {
    final c = SomaTheme.of(context);
    return ComponentPage(
      title: 'Skeleton',
      subtitle: 'Pulsing placeholder for content loading states.',
      preview: Column(
        mainAxisSize: MainAxisSize.min,
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          Row(
            crossAxisAlignment: CrossAxisAlignment.start,
            children: [
              _loaded
                  ? CircleAvatar(
                      radius: 24,
                      backgroundColor: c.primary,
                      child: Text(
                        'S',
                        style: TextStyle(
                          fontFamily: 'Rajdhani',
                          fontSize: 18,
                          fontWeight: FontWeight.w700,
                          color: c.primaryForeground,
                        ),
                      ),
                    )
                  : const SomaSkeleton(width: 48, height: 48, borderRadius: 999),
              const SizedBox(width: 12),
              Expanded(
                child: Column(
                  crossAxisAlignment: CrossAxisAlignment.start,
                  children: _loaded
                      ? [
                          Text(
                            'Soma User',
                            style: TextStyle(
                              fontFamily: 'Outfit',
                              fontSize: 14,
                              fontWeight: FontWeight.w600,
                              color: c.foreground,
                            ),
                          ),
                          const SizedBox(height: 4),
                          Text(
                            'dev@kreesalis.com',
                            style: TextStyle(
                              fontFamily: 'Outfit',
                              fontSize: 12,
                              color: c.mutedForeground,
                            ),
                          ),
                        ]
                      : [
                          const SomaSkeleton(height: 14),
                          const SizedBox(height: 8),
                          const SomaSkeleton(width: 140, height: 12),
                        ],
                ),
              ),
            ],
          ),
          const SizedBox(height: 16),
          ...(_loaded
              ? [
                  Text(
                    'This is the loaded content that replaces skeleton lines.',
                    style: TextStyle(
                      fontFamily: 'Outfit',
                      fontSize: 13,
                      color: c.foreground,
                    ),
                  ),
                ]
              : [
                  const SomaSkeleton(height: 12),
                  const SizedBox(height: 8),
                  const SomaSkeleton(height: 12),
                  const SizedBox(height: 8),
                  const SomaSkeleton(width: 180, height: 12),
                ]),
        ],
      ),
      controls: Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          Row(children: [
            SizedBox(
              width: 120,
              child: Text(
                'Loaded',
                style: TextStyle(
                  color: c.mutedForeground,
                  fontFamily: 'Outfit',
                  fontSize: 13,
                ),
              ),
            ),
            SomaSwitch(
              value: _loaded,
              onChanged: (v) => setState(() => _loaded = v),
            ),
          ]),
        ],
      ),
    );
  }
}
