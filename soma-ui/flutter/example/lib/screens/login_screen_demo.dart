import 'package:flutter/material.dart';
import 'package:soma_ui/soma_ui.dart';
import '../component_page.dart';

class LoginScreenDemo extends StatelessWidget {
  const LoginScreenDemo({super.key});

  @override
  Widget build(BuildContext context) {
    return ComponentPage(
      title: 'Login Screen',
      subtitle: 'Palantir-style two-column authentication screen',
      preview: const SizedBox(height: 500, child: SomaLoginScreen()),
      controls: const SizedBox.shrink(),
    );
  }
}
