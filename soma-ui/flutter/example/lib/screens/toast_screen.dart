import 'package:flutter/material.dart';
import 'package:soma_ui/soma_ui.dart';
import '../component_page.dart';

class ToastScreen extends StatefulWidget {
  const ToastScreen({super.key});

  @override
  State<ToastScreen> createState() => _ToastScreenState();
}

class _ToastScreenState extends State<ToastScreen> {
  SomaToastVariant _variant = SomaToastVariant.normal;
  bool _showDescription = true;

  void _show(SomaToastVariant v) {
    showSomaToast(
      context,
      message: '${v.name[0].toUpperCase()}${v.name.substring(1)} toast',
      description: _showDescription ? 'This is the supporting description.' : null,
      variant: v,
    );
  }

  @override
  Widget build(BuildContext context) {
    final c = SomaTheme.of(context);
    return ComponentPage(
      title: 'Toast',
      subtitle: 'Stacked overlay notifications — covers Toast and Sonner.',
      preview: Column(
        mainAxisSize: MainAxisSize.min,
        children: [
          SomaButton(
            onPressed: () => _show(_variant),
            child: const Text('Show toast'),
          ),
          const SizedBox(height: 24),
          Text(
            'All Variants',
            style: TextStyle(
              fontFamily: 'Outfit',
              fontSize: 13,
              color: c.mutedForeground,
            ),
          ),
          const SizedBox(height: 12),
          Wrap(
            spacing: 8,
            runSpacing: 8,
            alignment: WrapAlignment.center,
            children: SomaToastVariant.values
                .map(
                  (v) => SomaButton(
                    variant: SomaButtonVariant.outline,
                    onPressed: () => _show(v),
                    child: Text(v.name),
                  ),
                )
                .toList(),
          ),
          const SizedBox(height: 12),
          SomaButton(
            variant: SomaButtonVariant.secondary,
            onPressed: () {
              for (final v in SomaToastVariant.values) {
                Future.delayed(
                  Duration(milliseconds: v.index * 150),
                  () {
                    if (mounted) _show(v);
                  },
                );
              }
            },
            child: const Text('Show many (stack demo)'),
          ),
        ],
      ),
      controls: Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          Row(children: [
            SizedBox(
              width: 120,
              child: Text(
                'Variant',
                style: TextStyle(
                  color: c.mutedForeground,
                  fontFamily: 'Outfit',
                  fontSize: 13,
                ),
              ),
            ),
            Expanded(
              child: SomaSelect<SomaToastVariant>(
                items: SomaToastVariant.values
                    .map((v) => SomaSelectItem(value: v, label: v.name))
                    .toList(),
                value: _variant,
                onChanged: (v) => setState(() => _variant = v),
              ),
            ),
          ]),
          const SizedBox(height: 12),
          Row(children: [
            SizedBox(
              width: 120,
              child: Text(
                'Description',
                style: TextStyle(
                  color: c.mutedForeground,
                  fontFamily: 'Outfit',
                  fontSize: 13,
                ),
              ),
            ),
            SomaSwitch(
              value: _showDescription,
              onChanged: (v) => setState(() => _showDescription = v),
            ),
          ]),
        ],
      ),
    );
  }
}
