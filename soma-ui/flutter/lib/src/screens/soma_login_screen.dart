import 'package:flutter/material.dart';
import 'package:soma_ui/soma_ui.dart';

class SomaLoginScreen extends StatefulWidget {
  const SomaLoginScreen({super.key});
  @override
  State<SomaLoginScreen> createState() => _SomaLoginScreenState();
}

class _SomaLoginScreenState extends State<SomaLoginScreen> {
  final _emailCtrl = TextEditingController();
  final _passCtrl = TextEditingController();

  @override
  void dispose() {
    _emailCtrl.dispose();
    _passCtrl.dispose();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    final c = SomaTheme.of(context);
    return LayoutBuilder(builder: (context, constraints) {
      final isWide = constraints.maxWidth >= 600;
      if (isWide) {
        return Row(children: [
          SizedBox(width: 280, child: _BrandPanel(c: c)),
          Expanded(child: _FormPanel(c: c, emailCtrl: _emailCtrl, passCtrl: _passCtrl)),
        ]);
      }
      return _FormPanel(c: c, emailCtrl: _emailCtrl, passCtrl: _passCtrl, showMiniHeader: true);
    });
  }
}

class _BrandPanel extends StatelessWidget {
  final SomaColors c;
  const _BrandPanel({required this.c});

  @override
  Widget build(BuildContext context) {
    return Container(
      color: c.card,
      padding: const EdgeInsets.all(32),
      child: Column(
        mainAxisAlignment: MainAxisAlignment.spaceBetween,
        children: [
          Column(
            mainAxisSize: MainAxisSize.min,
            children: [
              Icon(LucideIcons.hexagon, size: 64, color: c.primary),
              const SizedBox(height: 16),
              Text('FOUNDRY', style: TextStyle(fontFamily: 'Rajdhani', fontSize: 36, fontWeight: FontWeight.w700, color: c.foreground)),
              const SizedBox(height: 8),
              Text('DATA OPERATIONS PLATFORM', style: TextStyle(fontFamily: 'Outfit', fontSize: 12, color: c.mutedForeground)),
            ],
          ),
          Row(
            mainAxisSize: MainAxisSize.min,
            children: [
              Container(width: 8, height: 8, decoration: const BoxDecoration(color: Color(0xFF22c55e), shape: BoxShape.circle)),
              const SizedBox(width: 8),
              Text('SYSTEM ONLINE', style: TextStyle(fontFamily: 'Outfit', fontSize: 12, color: c.mutedForeground)),
            ],
          ),
        ],
      ),
    );
  }
}

class _FormPanel extends StatelessWidget {
  final SomaColors c;
  final TextEditingController emailCtrl;
  final TextEditingController passCtrl;
  final bool showMiniHeader;

  const _FormPanel({
    required this.c,
    required this.emailCtrl,
    required this.passCtrl,
    this.showMiniHeader = false,
  });

  @override
  Widget build(BuildContext context) {
    return Container(
      color: c.background,
      child: SingleChildScrollView(
        padding: const EdgeInsets.all(32),
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            if (showMiniHeader) ...[
              Row(children: [
                Icon(LucideIcons.hexagon, size: 20, color: c.primary),
                const SizedBox(width: 8),
                Text('FOUNDRY', style: TextStyle(fontFamily: 'Rajdhani', fontSize: 18, fontWeight: FontWeight.w700, color: c.foreground)),
              ]),
              const SizedBox(height: 24),
            ],
            Text('Sign in', style: TextStyle(fontFamily: 'Rajdhani', fontSize: 28, fontWeight: FontWeight.w700, color: c.foreground)),
            const SizedBox(height: 8),
            Text('Enter your credentials to access Foundry', style: TextStyle(fontFamily: 'Outfit', fontSize: 14, color: c.mutedForeground)),
            const SizedBox(height: 16),
            SomaInput(placeholder: 'Email', controller: emailCtrl, keyboardType: TextInputType.emailAddress),
            const SizedBox(height: 8),
            SomaInput(placeholder: 'Password', controller: passCtrl, obscureText: true),
            const SizedBox(height: 16),
            SizedBox(width: double.infinity, child: SomaButton(variant: SomaButtonVariant.primary, onPressed: () {}, child: const Text('Sign in'))),
            const SizedBox(height: 16),
            Row(children: [
              const Expanded(child: Divider()),
              Padding(
                padding: const EdgeInsets.symmetric(horizontal: 12),
                child: Text('OR', style: TextStyle(fontFamily: 'Outfit', fontSize: 12, color: c.mutedForeground)),
              ),
              const Expanded(child: Divider()),
            ]),
            const SizedBox(height: 16),
            SizedBox(width: double.infinity, child: SomaButton(variant: SomaButtonVariant.outline, onPressed: () {}, child: const Text('Continue with SSO'))),
            const SizedBox(height: 32),
            Center(child: Text('Contact your administrator for access', style: TextStyle(fontFamily: 'Outfit', fontSize: 12, color: c.mutedForeground))),
          ],
        ),
      ),
    );
  }
}
