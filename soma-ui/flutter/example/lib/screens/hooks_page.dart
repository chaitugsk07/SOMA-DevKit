import 'dart:async';
import 'package:flutter/material.dart';
import 'package:flutter/services.dart';
import 'package:shared_preferences/shared_preferences.dart';
import 'package:soma_ui/soma_ui.dart';

class HooksPage extends StatefulWidget {
  const HooksPage({super.key});

  @override
  State<HooksPage> createState() => _HooksPageState();
}

class _HooksPageState extends State<HooksPage> {
  // useToggle
  bool _toggleValue = false;

  // useClipboard
  final _clipCtrl = TextEditingController();
  bool _copied = false;
  Timer? _copyTimer;

  // useLocale
  TextDirection _direction = TextDirection.ltr;

  // useLocalStorage
  final _storageCtrl = TextEditingController();
  static const _storageKey = 'hooks_demo_text';

  @override
  void initState() {
    super.initState();
    _loadStoredText();
  }

  Future<void> _loadStoredText() async {
    final prefs = await SharedPreferences.getInstance();
    final saved = prefs.getString(_storageKey) ?? '';
    if (mounted) {
      setState(() => _storageCtrl.text = saved);
    }
  }

  Future<void> _saveText(String value) async {
    final prefs = await SharedPreferences.getInstance();
    await prefs.setString(_storageKey, value);
  }

  @override
  void dispose() {
    _clipCtrl.dispose();
    _storageCtrl.dispose();
    _copyTimer?.cancel();
    super.dispose();
  }

  void _doCopy() {
    Clipboard.setData(ClipboardData(text: _clipCtrl.text));
    setState(() => _copied = true);
    _copyTimer?.cancel();
    _copyTimer = Timer(const Duration(milliseconds: 1500), () {
      if (mounted) setState(() => _copied = false);
    });
  }

  @override
  Widget build(BuildContext context) {
    final c = SomaTheme.of(context);
    final size = MediaQuery.sizeOf(context);
    final isWide = size.width >= 768;
    final brightness = Theme.of(context).brightness;

    return SingleChildScrollView(
      padding: const EdgeInsets.all(24),
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          Text('Hooks', style: Theme.of(context).textTheme.headlineMedium),
          const SizedBox(height: 4),
          Text(
            'Reactive utilities (Flutter-native equivalents of the web hooks).',
            style: TextStyle(
              fontFamily: 'Outfit',
              fontSize: 14,
              color: c.mutedForeground,
            ),
          ),
          const SizedBox(height: 24),

          // useToggle
          _HookCard(
            heading: 'useToggle',
            description: 'Boolean state with a flip function.',
            signature: 'bool value, VoidCallback toggle',
            demo: Row(
              children: [
                SomaSwitch(
                  value: _toggleValue,
                  onChanged: (v) => setState(() => _toggleValue = v),
                ),
                const SizedBox(width: 12),
                Text(
                  _toggleValue ? 'true' : 'false',
                  style: TextStyle(
                    fontFamily: 'Outfit',
                    fontSize: 14,
                    color: c.foreground,
                  ),
                ),
              ],
            ),
          ),

          const SizedBox(height: 16),

          // useWindowSize
          _HookCard(
            heading: 'useWindowSize',
            description:
                'Tracks the current window/viewport dimensions, live.',
            signature: 'Size size',
            demo: Text(
              '${size.width.toStringAsFixed(0)} × '
              '${size.height.toStringAsFixed(0)} px',
              style: TextStyle(
                fontFamily: 'Outfit',
                fontSize: 14,
                color: c.foreground,
              ),
            ),
          ),

          const SizedBox(height: 16),

          // useMediaQuery
          _HookCard(
            heading: 'useMediaQuery',
            description: 'Derives a boolean from a breakpoint condition.',
            signature: 'bool matches',
            demo: Row(
              children: [
                Container(
                  width: 10,
                  height: 10,
                  decoration: BoxDecoration(
                    color: isWide ? c.success : c.mutedForeground,
                    shape: BoxShape.circle,
                  ),
                ),
                const SizedBox(width: 8),
                Text(
                  'width >= 768 → ${isWide ? 'true' : 'false'}',
                  style: TextStyle(
                    fontFamily: 'Outfit',
                    fontSize: 14,
                    color: c.foreground,
                  ),
                ),
              ],
            ),
          ),

          const SizedBox(height: 16),

          // useClipboard
          _HookCard(
            heading: 'useClipboard',
            description: 'Writes text to the clipboard with a confirmation.',
            signature:
                'Future<void> copy(String text), bool isCopied',
            demo: Row(
              children: [
                Expanded(
                  child: SomaInput(
                    controller: _clipCtrl,
                    placeholder: 'Text to copy…',
                  ),
                ),
                const SizedBox(width: 8),
                SomaButton(
                  onPressed: _doCopy,
                  child: Text(_copied ? 'Copied!' : 'Copy'),
                ),
              ],
            ),
          ),

          const SizedBox(height: 16),

          // useTheme
          _HookCard(
            heading: 'useTheme',
            description:
                'Reads the current color scheme / brightness from context.',
            signature: 'Brightness brightness, SomaColors colors',
            demo: Row(
              children: [
                Icon(
                  brightness == Brightness.dark
                      ? LucideIcons.moon
                      : LucideIcons.sun,
                  size: 16,
                  color: c.foreground,
                ),
                const SizedBox(width: 8),
                Text(
                  brightness == Brightness.dark ? 'dark' : 'light',
                  style: TextStyle(
                    fontFamily: 'Outfit',
                    fontSize: 14,
                    color: c.foreground,
                  ),
                ),
                const SizedBox(width: 16),
                Text(
                  '(toggle in app bar ↑)',
                  style: TextStyle(
                    fontFamily: 'Outfit',
                    fontSize: 12,
                    color: c.mutedForeground,
                  ),
                ),
              ],
            ),
          ),

          const SizedBox(height: 16),

          // useLocale
          _HookCard(
            heading: 'useLocale',
            description:
                'Controls text directionality (LTR / RTL) at runtime.',
            signature: 'TextDirection direction',
            demo: Column(
              crossAxisAlignment: CrossAxisAlignment.start,
              children: [
                Row(
                  children: [
                    _DirButton(
                      label: 'LTR',
                      active: _direction == TextDirection.ltr,
                      onTap: () =>
                          setState(() => _direction = TextDirection.ltr),
                    ),
                    const SizedBox(width: 8),
                    _DirButton(
                      label: 'RTL',
                      active: _direction == TextDirection.rtl,
                      onTap: () =>
                          setState(() => _direction = TextDirection.rtl),
                    ),
                  ],
                ),
                const SizedBox(height: 12),
                Directionality(
                  textDirection: _direction,
                  child: Text(
                    'The quick brown fox jumps over the lazy dog.',
                    style: TextStyle(
                      fontFamily: 'Outfit',
                      fontSize: 14,
                      color: c.foreground,
                    ),
                  ),
                ),
              ],
            ),
          ),

          const SizedBox(height: 16),

          // useLocalStorage
          _HookCard(
            heading: 'useLocalStorage',
            description:
                'Persists a string value across reloads via shared_preferences '
                '(localStorage on web, NSUserDefaults on iOS/macOS, '
                'SharedPreferences on Android).',
            signature:
                'T value, Future<void> set(T value)',
            demo: Column(
              crossAxisAlignment: CrossAxisAlignment.start,
              children: [
                SomaInput(
                  controller: _storageCtrl,
                  placeholder: 'Type something — it persists…',
                  onChanged: _saveText,
                ),
                const SizedBox(height: 8),
                Text(
                  'Reload the app to confirm persistence.',
                  style: TextStyle(
                    fontFamily: 'Outfit',
                    fontSize: 12,
                    color: c.mutedForeground,
                  ),
                ),
              ],
            ),
          ),

          const SizedBox(height: 24),
        ],
      ),
    );
  }
}

class _HookCard extends StatelessWidget {
  final String heading;
  final String description;
  final String signature;
  final Widget demo;

  const _HookCard({
    required this.heading,
    required this.description,
    required this.signature,
    required this.demo,
  });

  @override
  Widget build(BuildContext context) {
    final c = SomaTheme.of(context);
    return SomaCard(
      padding: const EdgeInsets.all(20),
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          Text(
            heading,
            style: TextStyle(
              fontFamily: 'Rajdhani',
              fontSize: 18,
              fontWeight: FontWeight.w600,
              color: c.cardForeground,
            ),
          ),
          const SizedBox(height: 4),
          Text(
            description,
            style: TextStyle(
              fontFamily: 'Outfit',
              fontSize: 13,
              color: c.mutedForeground,
            ),
          ),
          const SizedBox(height: 16),
          demo,
          const SizedBox(height: 16),
          Container(
            padding: const EdgeInsets.symmetric(horizontal: 12, vertical: 8),
            decoration: BoxDecoration(
              color: c.muted,
              borderRadius: BorderRadius.circular(4),
            ),
            child: Text(
              signature,
              style: TextStyle(
                fontFamily: 'monospace',
                fontSize: 12,
                color: c.mutedForeground,
              ),
            ),
          ),
        ],
      ),
    );
  }
}

class _DirButton extends StatelessWidget {
  final String label;
  final bool active;
  final VoidCallback onTap;

  const _DirButton({
    required this.label,
    required this.active,
    required this.onTap,
  });

  @override
  Widget build(BuildContext context) {
    return SomaButton(
      variant: active ? SomaButtonVariant.primary : SomaButtonVariant.outline,
      size: SomaButtonSize.sm,
      onPressed: onTap,
      child: Text(label),
    );
  }
}
