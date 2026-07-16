import 'package:flutter/material.dart';
import 'package:soma_ui/soma_ui.dart';
import '../component_page.dart';

class DrawerScreen extends StatelessWidget {
  const DrawerScreen({super.key});

  @override
  Widget build(BuildContext context) {
    final c = SomaTheme.of(context);
    return ComponentPage(
      title: 'Drawer',
      subtitle: 'Bottom sheet panel for mobile-friendly overlays.',
      preview: SomaButton(
        onPressed: () => showSomaDrawer(
          context,
          builder: (ctx) => Column(
            crossAxisAlignment: CrossAxisAlignment.start,
            mainAxisSize: MainAxisSize.min,
            children: [
              SomaDrawerHeader(
                child: Column(
                  crossAxisAlignment: CrossAxisAlignment.start,
                  children: [
                    const SomaDrawerTitle(text: 'Edit Profile'),
                    const SizedBox(height: 4),
                    SomaDrawerDescription(
                      text: 'Make changes to your profile here.',
                    ),
                  ],
                ),
              ),
              SomaInput(
                placeholder: 'Display name',
                onChanged: (_) {},
              ),
              const SizedBox(height: 12),
              SomaInput(
                placeholder: 'Email address',
                onChanged: (_) {},
              ),
              SomaDrawerFooter(
                children: [
                  SomaButton(
                    variant: SomaButtonVariant.outline,
                    onPressed: () => Navigator.pop(ctx),
                    child: const Text('Cancel'),
                  ),
                  SomaButton(
                    onPressed: () => Navigator.pop(ctx),
                    child: const Text('Save changes'),
                  ),
                ],
              ),
            ],
          ),
        ),
        child: const Text('Open Drawer'),
      ),
      controls: Text(
        'Click the button to open the bottom drawer. Tap outside or drag down to dismiss.',
        style: TextStyle(
          fontFamily: 'Outfit',
          fontSize: 13,
          color: c.mutedForeground,
        ),
      ),
    );
  }
}
