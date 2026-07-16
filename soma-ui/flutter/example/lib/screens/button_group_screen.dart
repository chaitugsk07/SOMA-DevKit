import 'package:flutter/material.dart';
import 'package:soma_ui/soma_ui.dart';
import '../component_page.dart';

class ButtonGroupScreen extends StatelessWidget {
  const ButtonGroupScreen({super.key});

  @override
  Widget build(BuildContext context) {
    return ComponentPage(
      title: 'ButtonGroup',
      subtitle: 'Segmented control that merges child buttons into one unit.',
      preview: SomaButtonGroup(
        children: [
          SomaButton(
            variant: SomaButtonVariant.outline,
            onPressed: () {},
            child: const Text('Day'),
          ),
          SomaButton(
            variant: SomaButtonVariant.outline,
            onPressed: () {},
            child: const Text('Week'),
          ),
          SomaButton(
            variant: SomaButtonVariant.outline,
            onPressed: () {},
            child: const Text('Month'),
          ),
        ],
      ),
      controls: const _Info(),
    );
  }
}

class _Info extends StatelessWidget {
  const _Info();

  @override
  Widget build(BuildContext context) {
    final c = SomaTheme.of(context);
    return Text(
      'Pass any SomaButton children — borders are merged automatically.',
      style: TextStyle(
        fontFamily: 'Outfit',
        fontSize: 13,
        color: c.mutedForeground,
      ),
    );
  }
}
