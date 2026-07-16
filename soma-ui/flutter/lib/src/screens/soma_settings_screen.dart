import 'package:flutter/material.dart';
import 'package:soma_ui/soma_ui.dart';

class SomaSettingsScreen extends StatefulWidget {
  const SomaSettingsScreen({super.key});
  @override
  State<SomaSettingsScreen> createState() => _SomaSettingsScreenState();
}

class _SomaSettingsScreenState extends State<SomaSettingsScreen> {
  int _selectedNav = 0;
  bool _notifPipeline = true;
  bool _notifDigest = false;
  bool _notifEmail = true;
  bool _secTwoFactor = true;
  bool _secLogging = false;
  final _nameCtrl = TextEditingController(text: 'John Doe');
  final _emailCtrl = TextEditingController(text: 'john@example.com');

  @override
  void dispose() {
    _nameCtrl.dispose();
    _emailCtrl.dispose();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    final c = SomaTheme.of(context);
    return LayoutBuilder(builder: (context, constraints) {
      final isWide = constraints.maxWidth >= 600;
      final panel = _buildPanel(c);
      if (isWide) {
        return Row(children: [
          _SettingsNav(c: c, selected: _selectedNav, onSelect: (i) => setState(() => _selectedNav = i)),
          Expanded(child: SingleChildScrollView(padding: const EdgeInsets.all(20), child: panel)),
        ]);
      }
      return Column(children: [
        SomaTabs(
          tabs: const [
            SomaTab(label: 'Profile'),
            SomaTab(label: 'Notifications'),
            SomaTab(label: 'Security'),
            SomaTab(label: 'API Keys'),
          ],
          index: _selectedNav,
          onChanged: (i) => setState(() => _selectedNav = i),
        ),
        Expanded(child: SingleChildScrollView(padding: const EdgeInsets.all(16), child: panel)),
      ]);
    });
  }

  Widget _buildPanel(SomaColors c) {
    switch (_selectedNav) {
      case 0:
        return _ProfilePanel(c: c, nameCtrl: _nameCtrl, emailCtrl: _emailCtrl);
      case 1:
        return _NotificationsPanel(
          c: c,
          pipeline: _notifPipeline, onPipeline: (v) => setState(() => _notifPipeline = v),
          digest: _notifDigest, onDigest: (v) => setState(() => _notifDigest = v),
          email: _notifEmail, onEmail: (v) => setState(() => _notifEmail = v),
        );
      case 2:
        return _SecurityPanel(
          c: c,
          twoFactor: _secTwoFactor, onTwoFactor: (v) => setState(() => _secTwoFactor = v),
          logging: _secLogging, onLogging: (v) => setState(() => _secLogging = v),
        );
      case 3:
        return _ApiKeysPanel(c: c);
      default:
        return const SizedBox.shrink();
    }
  }
}

class _SettingsNav extends StatelessWidget {
  final SomaColors c;
  final int selected;
  final ValueChanged<int> onSelect;
  static const _items = ['Profile', 'Notifications', 'Security', 'API Keys'];

  const _SettingsNav({required this.c, required this.selected, required this.onSelect});

  @override
  Widget build(BuildContext context) {
    return Container(
      width: 200,
      decoration: BoxDecoration(color: c.card, border: Border(right: BorderSide(color: c.border))),
      child: Column(
        children: List.generate(_items.length, (i) {
          final isActive = i == selected;
          return GestureDetector(
            onTap: () => onSelect(i),
            child: Container(
              decoration: BoxDecoration(
                border: Border(left: BorderSide(color: isActive ? c.primary : Colors.transparent, width: 3)),
              ),
              padding: const EdgeInsets.symmetric(horizontal: 16, vertical: 12),
              child: Row(children: [
                Text(_items[i], style: TextStyle(fontFamily: 'Outfit', fontSize: 13, color: isActive ? c.primary : c.mutedForeground)),
              ]),
            ),
          );
        }),
      ),
    );
  }
}

class _ProfilePanel extends StatelessWidget {
  final SomaColors c;
  final TextEditingController nameCtrl;
  final TextEditingController emailCtrl;

  const _ProfilePanel({required this.c, required this.nameCtrl, required this.emailCtrl});

  @override
  Widget build(BuildContext context) {
    return SomaCard(
      child: Padding(
        padding: const EdgeInsets.all(16),
        child: Column(crossAxisAlignment: CrossAxisAlignment.start, children: [
          Text('Profile', style: TextStyle(fontFamily: 'Outfit', fontSize: 14, fontWeight: FontWeight.w600, color: c.foreground)),
          const SizedBox(height: 16),
          SomaInput(placeholder: 'Display Name', controller: nameCtrl),
          const SizedBox(height: 8),
          SomaInput(placeholder: 'Email', keyboardType: TextInputType.emailAddress, controller: emailCtrl),
          const SizedBox(height: 16),
          SizedBox(width: double.infinity, child: SomaButton(variant: SomaButtonVariant.primary, onPressed: () {}, child: const Text('Save changes'))),
        ]),
      ),
    );
  }
}

class _NotificationsPanel extends StatelessWidget {
  final SomaColors c;
  final bool pipeline;
  final ValueChanged<bool> onPipeline;
  final bool digest;
  final ValueChanged<bool> onDigest;
  final bool email;
  final ValueChanged<bool> onEmail;

  const _NotificationsPanel({required this.c, required this.pipeline, required this.onPipeline, required this.digest, required this.onDigest, required this.email, required this.onEmail});

  @override
  Widget build(BuildContext context) {
    return SomaCard(
      child: Padding(
        padding: const EdgeInsets.all(16),
        child: Column(crossAxisAlignment: CrossAxisAlignment.start, children: [
          Text('Notifications', style: TextStyle(fontFamily: 'Outfit', fontSize: 14, fontWeight: FontWeight.w600, color: c.foreground)),
          const SizedBox(height: 16),
          _SwitchRow(c: c, label: 'Pipeline notifications', value: pipeline, onChanged: onPipeline),
          const SomaSeparator(),
          _SwitchRow(c: c, label: 'Daily digest', value: digest, onChanged: onDigest),
          const SomaSeparator(),
          _SwitchRow(c: c, label: 'Alert emails', value: email, onChanged: onEmail),
          const SizedBox(height: 16),
          SizedBox(width: double.infinity, child: SomaButton(variant: SomaButtonVariant.primary, onPressed: () {}, child: const Text('Save changes'))),
        ]),
      ),
    );
  }
}

class _SecurityPanel extends StatelessWidget {
  final SomaColors c;
  final bool twoFactor;
  final ValueChanged<bool> onTwoFactor;
  final bool logging;
  final ValueChanged<bool> onLogging;

  const _SecurityPanel({required this.c, required this.twoFactor, required this.onTwoFactor, required this.logging, required this.onLogging});

  @override
  Widget build(BuildContext context) {
    return SomaCard(
      child: Padding(
        padding: const EdgeInsets.all(16),
        child: Column(crossAxisAlignment: CrossAxisAlignment.start, children: [
          Text('Security', style: TextStyle(fontFamily: 'Outfit', fontSize: 14, fontWeight: FontWeight.w600, color: c.foreground)),
          const SizedBox(height: 16),
          _SwitchRow(c: c, label: 'Two-factor authentication', value: twoFactor, onChanged: onTwoFactor),
          const SomaSeparator(),
          _SwitchRow(c: c, label: 'API request logging', value: logging, onChanged: onLogging),
          const SizedBox(height: 16),
          SizedBox(width: double.infinity, child: SomaButton(variant: SomaButtonVariant.primary, onPressed: () {}, child: const Text('Save changes'))),
        ]),
      ),
    );
  }
}

class _ApiKeysPanel extends StatelessWidget {
  final SomaColors c;
  const _ApiKeysPanel({required this.c});

  @override
  Widget build(BuildContext context) {
    return SomaCard(
      child: Padding(
        padding: const EdgeInsets.all(16),
        child: Column(crossAxisAlignment: CrossAxisAlignment.start, children: [
          Text('API Keys', style: TextStyle(fontFamily: 'Outfit', fontSize: 14, fontWeight: FontWeight.w600, color: c.foreground)),
          const SizedBox(height: 16),
          _ApiKeyRow(c: c, name: 'Production API Key', maskedKey: 'sk-...a7f2'),
          const SizedBox(height: 8),
          _ApiKeyRow(c: c, name: 'Development API Key', maskedKey: 'sk-...3c9e'),
          const SizedBox(height: 16),
          SizedBox(width: double.infinity, child: SomaButton(variant: SomaButtonVariant.primary, onPressed: () {}, child: const Text('+ Generate New Key'))),
        ]),
      ),
    );
  }
}

class _ApiKeyRow extends StatelessWidget {
  final SomaColors c;
  final String name;
  final String maskedKey;

  const _ApiKeyRow({required this.c, required this.name, required this.maskedKey});

  @override
  Widget build(BuildContext context) {
    return Row(children: [
      Expanded(child: Column(crossAxisAlignment: CrossAxisAlignment.start, children: [
        Text(name, style: TextStyle(fontFamily: 'Outfit', fontSize: 13, color: c.foreground)),
        Text(maskedKey, style: TextStyle(fontFamily: 'monospace', fontSize: 11, color: c.mutedForeground)),
      ])),
      SomaButton(variant: SomaButtonVariant.outline, size: SomaButtonSize.sm, onPressed: () {}, child: const Text('Revoke')),
    ]);
  }
}

class _SwitchRow extends StatelessWidget {
  final SomaColors c;
  final String label;
  final bool value;
  final ValueChanged<bool> onChanged;

  const _SwitchRow({required this.c, required this.label, required this.value, required this.onChanged});

  @override
  Widget build(BuildContext context) {
    return Padding(
      padding: const EdgeInsets.symmetric(vertical: 8),
      child: Row(children: [
        Expanded(child: Text(label, style: TextStyle(fontFamily: 'Outfit', fontSize: 13, color: c.foreground))),
        SomaSwitch(value: value, onChanged: onChanged),
      ]),
    );
  }
}
