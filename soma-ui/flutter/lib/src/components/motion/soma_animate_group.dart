import 'package:flutter/material.dart';
import 'soma_animate.dart';

// ponytail: each child is wrapped in a FutureBuilder that mounts it after
// index*stagger delay, then handed to SomaAnimate. No new dependencies;
// reuses SomaAnimate as-is.
// Ceiling: web AnimateGroup shares one CSS class; stagger here is per-widget
// mount-delay. If SomaAnimate ever gains a `delay` param, remove the
// _DelayedMount wrapper.

/// Staggered entrance animation for a list of children.
///
/// Mirrors the web soma-ui `AnimateGroup` in
/// `packages/ui/src/components/motion/animate_group.rs`,
/// but adds per-child stagger (the web version uses a uniform CSS class).
/// Reuses [SomaAnimate] for the entrance animation.
class SomaAnimateGroup extends StatelessWidget {
  final List<Widget> children;
  final SomaAnimationType animation;
  final Duration stagger;

  const SomaAnimateGroup({
    super.key,
    required this.children,
    this.animation = SomaAnimationType.fadeIn,
    this.stagger = const Duration(milliseconds: 80),
  });

  @override
  Widget build(BuildContext context) {
    return Column(
      mainAxisSize: MainAxisSize.min,
      children: [
        for (int i = 0; i < children.length; i++)
          _DelayedMount(
            delay: stagger * i,
            child: SomaAnimate(
              animation: animation,
              child: children[i],
            ),
          ),
      ],
    );
  }
}

/// Mounts [child] after [delay] elapses; shows nothing before then.
class _DelayedMount extends StatefulWidget {
  final Duration delay;
  final Widget child;

  const _DelayedMount({required this.delay, required this.child});

  @override
  State<_DelayedMount> createState() => _DelayedMountState();
}

class _DelayedMountState extends State<_DelayedMount> {
  bool _visible = false;

  @override
  void initState() {
    super.initState();
    if (widget.delay == Duration.zero) {
      _visible = true;
    } else {
      Future.delayed(widget.delay, () {
        if (mounted) setState(() => _visible = true);
      });
    }
  }

  @override
  Widget build(BuildContext context) {
    return _visible ? widget.child : const SizedBox.shrink();
  }
}
