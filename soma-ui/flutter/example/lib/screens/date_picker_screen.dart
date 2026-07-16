import 'package:flutter/material.dart';
import 'package:soma_ui/soma_ui.dart';
import '../component_page.dart';

class DatePickerScreen extends StatefulWidget {
  const DatePickerScreen({super.key});

  @override
  State<DatePickerScreen> createState() => _DatePickerScreenState();
}

class _DatePickerScreenState extends State<DatePickerScreen> {
  String? _date;

  @override
  Widget build(BuildContext context) {
    final c = SomaTheme.of(context);
    return ComponentPage(
      title: 'DatePicker',
      subtitle: 'Single-date popover calendar. ISO yyyy-MM-dd value.',
      preview: Column(
        mainAxisSize: MainAxisSize.min,
        children: [
          SizedBox(
            width: 280,
            child: SomaDatePicker(
              value: _date,
              onChanged: (v) => setState(() => _date = v),
            ),
          ),
          const SizedBox(height: 16),
          Text(
            _date == null ? 'No date selected' : 'Picked: $_date',
            style: TextStyle(
                fontFamily: 'Outfit', fontSize: 13, color: c.mutedForeground),
          ),
        ],
      ),
      controls: Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          Row(children: [
            SizedBox(
              width: 80,
              child: Text('Clear',
                  style: TextStyle(
                      color: c.mutedForeground,
                      fontFamily: 'Outfit',
                      fontSize: 13)),
            ),
            SomaButton(
              variant: SomaButtonVariant.outline,
              size: SomaButtonSize.sm,
              onPressed: () => setState(() => _date = null),
              child: const Text('Clear selection'),
            ),
          ]),
        ],
      ),
    );
  }
}
