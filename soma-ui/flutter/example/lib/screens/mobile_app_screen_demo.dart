import 'package:flutter/material.dart';
import 'package:soma_ui/soma_ui.dart';
import '../component_page.dart';

class MobileAppScreenDemo extends StatelessWidget {
  const MobileAppScreenDemo({super.key});

  @override
  Widget build(BuildContext context) {
    return ComponentPage(
      title: 'Mobile App Screen',
      subtitle: 'Phone-form-factor layout with bottom navigation',
      preview: SizedBox(
        height: 600,
        child: Center(
          child: SomaCard(
            child: SizedBox(
              width: 390,
              height: 580,
              child: ClipRRect(
                borderRadius: BorderRadius.circular(8),
                child: const SomaMobileAppScreen(),
              ),
            ),
          ),
        ),
      ),
      controls: const SizedBox.shrink(),
    );
  }
}
