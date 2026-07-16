import 'package:flutter/material.dart';
import 'package:soma_ui/soma_ui.dart';
import '../component_page.dart';

class AnimateScreen extends StatefulWidget {
  const AnimateScreen({super.key});

  @override
  State<AnimateScreen> createState() => _AnimateScreenState();
}

class _AnimateScreenState extends State<AnimateScreen> {
  SomaAnimationType _animation = SomaAnimationType.fadeIn;
  // Bumping this key re-mounts SomaAnimate, replaying the animation
  int _replayKey = 0;

  @override
  Widget build(BuildContext context) {
    final c = SomaTheme.of(context);
    return ComponentPage(
      title: 'Animate',
      subtitle: 'Entrance animation wrapper with configurable animation types.',
      preview: Column(
        mainAxisSize: MainAxisSize.min,
        children: [
          SomaAnimate(
            key: ValueKey(_replayKey),
            animation: _animation,
            child: SomaCard(
              padding: const EdgeInsets.symmetric(horizontal: 24, vertical: 16),
              child: Text(
                'Animated content',
                style: TextStyle(
                  fontFamily: 'Outfit',
                  fontSize: 15,
                  color: c.foreground,
                ),
              ),
            ),
          ),
          const SizedBox(height: 16),
          SomaButton(
            onPressed: () => setState(() => _replayKey++),
            child: const Text('Replay'),
          ),
        ],
      ),
      controls: Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          Row(children: [
            SizedBox(
              width: 120,
              child: Text(
                'Animation',
                style: TextStyle(
                  color: c.mutedForeground,
                  fontFamily: 'Outfit',
                  fontSize: 13,
                ),
              ),
            ),
            Expanded(
              child: SomaSelect<SomaAnimationType>(
                items: SomaAnimationType.values
                    .map((v) => SomaSelectItem(value: v, label: v.name))
                    .toList(),
                value: _animation,
                onChanged: (v) => setState(() {
                  _animation = v;
                  _replayKey++;
                }),
              ),
            ),
          ]),
        ],
      ),
    );
  }
}
