import 'package:flutter/material.dart';
import 'package:soma_ui/soma_ui.dart';
import '../component_page.dart';

class AnimateGroupScreen extends StatefulWidget {
  const AnimateGroupScreen({super.key});

  @override
  State<AnimateGroupScreen> createState() => _AnimateGroupScreenState();
}

class _AnimateGroupScreenState extends State<AnimateGroupScreen> {
  SomaAnimationType _animation = SomaAnimationType.slideUp;
  int _key = 0; // increment to replay

  final List<String> _cardLabels = [
    'Alpha',
    'Beta',
    'Gamma',
    'Delta',
    'Epsilon',
    'Zeta',
  ];

  @override
  Widget build(BuildContext context) {
    final c = SomaTheme.of(context);
    return ComponentPage(
      title: 'Animate Group',
      subtitle: 'Staggered entrance animation for a list of children.',
      preview: KeyedSubtree(
        key: ValueKey(_key),
        child: SomaAnimateGroup(
          animation: _animation,
          stagger: const Duration(milliseconds: 80),
          children: [
            for (final label in _cardLabels)
              Padding(
                padding: const EdgeInsets.only(bottom: 8),
                child: Container(
                  padding: const EdgeInsets.symmetric(horizontal: 16, vertical: 12),
                  decoration: BoxDecoration(
                    color: c.card,
                    border: Border.all(color: c.border),
                    borderRadius: BorderRadius.circular(6),
                  ),
                  child: Row(
                    mainAxisSize: MainAxisSize.min,
                    children: [
                      Container(
                        width: 8,
                        height: 8,
                        decoration: BoxDecoration(
                          color: c.primary,
                          shape: BoxShape.circle,
                        ),
                      ),
                      const SizedBox(width: 10),
                      Text(
                        label,
                        style: TextStyle(
                          fontFamily: 'Outfit',
                          fontSize: 14,
                          color: c.foreground,
                        ),
                      ),
                    ],
                  ),
                ),
              ),
          ],
        ),
      ),
      controls: Column(
        children: [
          ControlRow(
            label: 'Animation',
            child: SomaSelect<SomaAnimationType>(
              items: SomaAnimationType.values
                  .map((v) => SomaSelectItem(value: v, label: v.name))
                  .toList(),
              value: _animation,
              onChanged: (v) => setState(() {
                _animation = v;
                _key++;
              }),
            ),
          ),
          ControlRow(
            label: 'Replay',
            child: SomaButton(
              variant: SomaButtonVariant.outline,
              size: SomaButtonSize.sm,
              onPressed: () => setState(() => _key++),
              child: const Text('Replay'),
            ),
          ),
        ],
      ),
    );
  }
}
