import 'package:flutter/material.dart';
import 'package:soma_ui/soma_ui.dart';
import '../component_page.dart';

class AvatarScreen extends StatefulWidget {
  const AvatarScreen({super.key});

  @override
  State<AvatarScreen> createState() => _AvatarScreenState();
}

class _AvatarScreenState extends State<AvatarScreen> {
  SomaAvatarSize _size = SomaAvatarSize.md;

  @override
  Widget build(BuildContext context) {
    final c = SomaTheme.of(context);
    return ComponentPage(
      title: 'Avatar',
      subtitle: 'User avatar with image or initials fallback.',
      preview: Column(
        mainAxisSize: MainAxisSize.min,
        children: [
          SomaAvatar(initials: 'AB', size: _size),
          const SizedBox(height: 16),
          Row(
            mainAxisSize: MainAxisSize.min,
            children: SomaAvatarSize.values.map((s) => Padding(
              padding: const EdgeInsets.symmetric(horizontal: 4),
              child: SomaAvatar(initials: 'AB', size: s),
            )).toList(),
          ),
        ],
      ),
      controls: Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          Row(children: [
            SizedBox(width: 120, child: Text('Size', style: TextStyle(color: c.mutedForeground, fontFamily: 'Outfit', fontSize: 13))),
            Expanded(
              child: SomaSelect<SomaAvatarSize>(
                items: SomaAvatarSize.values.map((v) => SomaSelectItem(value: v, label: v.name)).toList(),
                value: _size,
                onChanged: (v) => setState(() => _size = v),
              ),
            ),
          ]),
        ],
      ),
    );
  }
}
