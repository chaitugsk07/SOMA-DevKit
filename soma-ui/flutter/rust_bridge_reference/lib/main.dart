import 'package:flutter/material.dart';
import 'package:soma_ui/soma_ui.dart';
import 'package:rust_bridge_reference/src/rust/api/simple.dart';
import 'package:rust_bridge_reference/src/rust/frb_generated.dart';

Future<void> main() async {
  await RustLib.init();
  runApp(const MyApp());
}

class MyApp extends StatelessWidget {
  const MyApp({super.key});

  @override
  Widget build(BuildContext context) {
    return SomaThemeProvider(
      themeMode: ThemeMode.dark,
      child: _MaterialShell(),
    );
  }
}

/// Reads soma theme and passes it into MaterialApp.
class _MaterialShell extends StatelessWidget {
  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      debugShowCheckedModeBanner: false,
      theme: SomaTheme.buildThemeData(context),
      home: const BridgeDemoScreen(),
    );
  }
}

class BridgeDemoScreen extends StatefulWidget {
  const BridgeDemoScreen({super.key});

  @override
  State<BridgeDemoScreen> createState() => _BridgeDemoScreenState();
}

class _BridgeDemoScreenState extends State<BridgeDemoScreen> {
  final _emailCtrl = TextEditingController();
  final _nameCtrl = TextEditingController();
  String _emailResult = '';
  String _greetResult = '';

  Future<void> _onValidate() async {
    final result = await validateEmail(email: _emailCtrl.text);
    setState(() => _emailResult = result ? 'Valid ✓' : 'Invalid ✗');
  }

  Future<void> _onGreet() async {
    final result = await greet(name: _nameCtrl.text);
    setState(() => _greetResult = result);
  }

  @override
  void dispose() {
    _emailCtrl.dispose();
    _nameCtrl.dispose();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    final c = SomaTheme.of(context);
    return Scaffold(
      appBar: AppBar(title: const Text('Flutter · Rust Bridge Reference')),
      body: Center(
        child: ConstrainedBox(
          constraints: const BoxConstraints(maxWidth: 480),
          child: Padding(
            padding: const EdgeInsets.all(24),
            child: Column(
              mainAxisSize: MainAxisSize.min,
              crossAxisAlignment: CrossAxisAlignment.stretch,
              children: [
                // Email section
                Text('Email Validator', style: Theme.of(context).textTheme.titleMedium),
                const SizedBox(height: 8),
                SomaInput(
                  controller: _emailCtrl,
                  placeholder: 'Enter email address',
                  keyboardType: TextInputType.emailAddress,
                ),
                const SizedBox(height: 8),
                SomaButton(
                  onPressed: _onValidate,
                  child: const Text('Validate (in Rust)'),
                ),
                if (_emailResult.isNotEmpty) ...[
                  const SizedBox(height: 8),
                  Text(_emailResult, style: TextStyle(color: c.mutedForeground)),
                ],

                const SizedBox(height: 32),

                // Greet section
                Text('Greeter', style: Theme.of(context).textTheme.titleMedium),
                const SizedBox(height: 8),
                SomaInput(
                  controller: _nameCtrl,
                  placeholder: 'Enter your name',
                ),
                const SizedBox(height: 8),
                SomaButton(
                  onPressed: _onGreet,
                  child: const Text('Greet (in Rust)'),
                ),
                if (_greetResult.isNotEmpty) ...[
                  const SizedBox(height: 8),
                  Text(_greetResult, style: TextStyle(color: c.mutedForeground)),
                ],

                const SizedBox(height: 40),

                // Caption
                Text(
                  'UI = Flutter · logic = Rust core via flutter_rust_bridge',
                  style: TextStyle(
                    color: c.mutedForeground,
                    fontSize: 12,
                  ),
                  textAlign: TextAlign.center,
                ),
              ],
            ),
          ),
        ),
      ),
    );
  }
}
