import 'package:flutter/material.dart';
import 'package:app/src/rust/api/simple.dart';
import 'package:app/models/view_models.dart';
import 'package:app/theme.dart';

class CountryPicker extends StatelessWidget {
  final Country selected;
  final ValueChanged<Country> onChanged;

  const CountryPicker({
    super.key,
    required this.selected,
    required this.onChanged,
  });

  @override
  Widget build(BuildContext context) {
    return SegmentedButton<Country>(
      segments: CountryInfo.all.map((info) {
        return ButtonSegment<Country>(
          value: info.country,
          label: Text('${info.flag} ${info.label}'),
        );
      }).toList(),
      selected: {selected},
      onSelectionChanged: (selection) => onChanged(selection.first),
      style: ButtonStyle(
        backgroundColor: WidgetStateProperty.resolveWith((states) {
          if (states.contains(WidgetState.selected)) {
            return AppTheme.gold.withAlpha(40);
          }
          return Colors.transparent;
        }),
        foregroundColor: WidgetStateProperty.resolveWith((states) {
          if (states.contains(WidgetState.selected)) {
            return AppTheme.gold;
          }
          return AppTheme.beige.withAlpha(160);
        }),
        side: WidgetStateProperty.all(
          BorderSide(color: AppTheme.gold.withAlpha(60)),
        ),
        shape: WidgetStateProperty.all(
          RoundedRectangleBorder(borderRadius: BorderRadius.circular(12)),
        ),
      ),
    );
  }
}
