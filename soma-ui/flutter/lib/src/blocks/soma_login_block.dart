import 'package:flutter/material.dart';
import 'package:soma_ui/soma_ui.dart';

class SomaLoginBlock extends StatefulWidget {
  const SomaLoginBlock({super.key});

  @override
  State<SomaLoginBlock> createState() => _SomaLoginBlockState();
}

class _SomaLoginBlockState extends State<SomaLoginBlock> {
  final _emailController = TextEditingController();
  final _passwordController = TextEditingController();

  @override
  void dispose() {
    _emailController.dispose();
    _passwordController.dispose();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    final c = SomaTheme.of(context);
    return Center(
      child: ConstrainedBox(
        constraints: const BoxConstraints(minHeight: 600, maxWidth: 360),
        child: Center(
          child: SomaCard(
            padding: const EdgeInsets.all(32),
            child: Column(
              mainAxisSize: MainAxisSize.min,
              crossAxisAlignment: CrossAxisAlignment.stretch,
              children: [
                // Header
                Text(
                  'Soma UI',
                  textAlign: TextAlign.center,
                  style: TextStyle(
                    fontFamily: 'Rajdhani',
                    fontSize: 24,
                    fontWeight: FontWeight.w700,
                    color: c.foreground,
                  ),
                ),
                const SizedBox(height: 4),
                Text(
                  'Sign in to your account',
                  textAlign: TextAlign.center,
                  style: TextStyle(
                    fontFamily: 'Outfit',
                    fontSize: 14,
                    color: c.mutedForeground,
                  ),
                ),
                const SizedBox(height: 24),
                // Email
                SomaInput(
                  placeholder: 'you@example.com',
                  controller: _emailController,
                  keyboardType: TextInputType.emailAddress,
                ),
                const SizedBox(height: 12),
                // Password
                SomaInput(
                  placeholder: '••••••••',
                  controller: _passwordController,
                  obscureText: true,
                ),
                const SizedBox(height: 16),
                // Sign in button
                SomaButton(
                  onPressed: () {},
                  child: const Text('Sign in'),
                ),
                const SizedBox(height: 16),
                // Divider with "or"
                Row(
                  children: [
                    const Expanded(child: Divider()),
                    Padding(
                      padding: const EdgeInsets.symmetric(horizontal: 12),
                      child: Text(
                        'or',
                        style: TextStyle(
                          fontFamily: 'Outfit',
                          fontSize: 12,
                          color: c.mutedForeground,
                        ),
                      ),
                    ),
                    const Expanded(child: Divider()),
                  ],
                ),
                const SizedBox(height: 16),
                // GitHub button
                SomaButton(
                  variant: SomaButtonVariant.outline,
                  onPressed: () {},
                  child: const Text('Continue with GitHub'),
                ),
                const SizedBox(height: 20),
                // Footer
                RichText(
                  textAlign: TextAlign.center,
                  text: TextSpan(
                    style: TextStyle(
                      fontFamily: 'Outfit',
                      fontSize: 13,
                      color: c.mutedForeground,
                    ),
                    children: [
                      const TextSpan(text: "Don't have an account? "),
                      TextSpan(
                        text: 'Sign up',
                        style: TextStyle(color: c.primary),
                      ),
                    ],
                  ),
                ),
              ],
            ),
          ),
        ),
      ),
    );
  }
}
