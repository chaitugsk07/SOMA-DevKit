import 'package:flutter/material.dart';
import 'package:soma_ui/soma_ui.dart';
import '../component_page.dart';

class LoginBlockScreen extends StatelessWidget {
  const LoginBlockScreen({super.key});

  @override
  Widget build(BuildContext context) {
    return ComponentPage(
      title: 'Login Block',
      subtitle: 'Centered sign-in card with email/password inputs.',
      preview: const SizedBox(
        width: double.infinity,
        child: SomaLoginBlock(),
      ),
      controls: const SizedBox.shrink(),
    );
  }
}
