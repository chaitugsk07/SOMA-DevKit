import 'package:flutter/material.dart';
import 'package:soma_ui/soma_ui.dart';
import '../component_page.dart';

class SheetScreen extends StatelessWidget {
  const SheetScreen({super.key});

  @override
  Widget build(BuildContext context) {
    final c = SomaTheme.of(context);
    return ComponentPage(
      title: 'Sheet',
      subtitle: 'Slide-in panel from any edge of the screen.',
      preview: Wrap(
        spacing: 12,
        runSpacing: 12,
        alignment: WrapAlignment.center,
        children: [
          SomaButton(
            variant: SomaButtonVariant.outline,
            onPressed: () => showSomaSheet(
              context,
              side: SomaSheetSide.right,
              builder: (ctx) => Column(
                crossAxisAlignment: CrossAxisAlignment.start,
                children: [
                  SomaSheetHeader(
                    child: Column(
                      crossAxisAlignment: CrossAxisAlignment.start,
                      children: [
                        const SomaSheetTitle(text: 'Right Sheet'),
                        const SizedBox(height: 4),
                        SomaSheetDescription(
                          text: 'Slides in from the right side.',
                        ),
                      ],
                    ),
                  ),
                  SomaSheetFooter(
                    children: [
                      SomaButton(
                        onPressed: () => Navigator.pop(ctx),
                        child: const Text('Close'),
                      ),
                    ],
                  ),
                ],
              ),
            ),
            child: const Text('Right'),
          ),
          SomaButton(
            variant: SomaButtonVariant.outline,
            onPressed: () => showSomaSheet(
              context,
              side: SomaSheetSide.left,
              builder: (ctx) => Column(
                crossAxisAlignment: CrossAxisAlignment.start,
                children: [
                  SomaSheetHeader(
                    child: Column(
                      crossAxisAlignment: CrossAxisAlignment.start,
                      children: [
                        const SomaSheetTitle(text: 'Left Sheet'),
                        const SizedBox(height: 4),
                        SomaSheetDescription(
                          text: 'Slides in from the left side.',
                        ),
                      ],
                    ),
                  ),
                  SomaSheetFooter(
                    children: [
                      SomaButton(
                        onPressed: () => Navigator.pop(ctx),
                        child: const Text('Close'),
                      ),
                    ],
                  ),
                ],
              ),
            ),
            child: const Text('Left'),
          ),
          SomaButton(
            variant: SomaButtonVariant.outline,
            onPressed: () => showSomaSheet(
              context,
              side: SomaSheetSide.bottom,
              builder: (ctx) => Column(
                crossAxisAlignment: CrossAxisAlignment.start,
                children: [
                  SomaSheetHeader(
                    child: Column(
                      crossAxisAlignment: CrossAxisAlignment.start,
                      children: [
                        const SomaSheetTitle(text: 'Bottom Sheet'),
                        const SizedBox(height: 4),
                        SomaSheetDescription(
                          text: 'Slides in from the bottom.',
                        ),
                      ],
                    ),
                  ),
                  SomaSheetFooter(
                    children: [
                      SomaButton(
                        onPressed: () => Navigator.pop(ctx),
                        child: const Text('Close'),
                      ),
                    ],
                  ),
                ],
              ),
            ),
            child: const Text('Bottom'),
          ),
        ],
      ),
      controls: Text(
        'Click a button to open a sheet from that edge. Press Escape or click the × to close.',
        style: TextStyle(
          fontFamily: 'Outfit',
          fontSize: 13,
          color: c.mutedForeground,
        ),
      ),
    );
  }
}
