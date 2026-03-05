import 'package:flutter/material.dart';
import 'package:app/theme.dart';

class AppSearchBar extends StatelessWidget {
  final TextEditingController controller;
  final VoidCallback onSubmitted;
  final VoidCallback? onClear;
  final String hintText;

  const AppSearchBar({
    super.key,
    required this.controller,
    required this.onSubmitted,
    this.onClear,
    this.hintText = 'Search for a team...',
  });

  @override
  Widget build(BuildContext context) {
    return TextField(
      controller: controller,
      style: const TextStyle(color: AppTheme.beige, fontSize: 16),
      cursorColor: AppTheme.gold,
      textInputAction: TextInputAction.search,
      onSubmitted: (_) => onSubmitted(),
      decoration: InputDecoration(
        hintText: hintText,
        hintStyle: TextStyle(color: AppTheme.beige.withAlpha(100)),
        prefixIcon: Icon(
          Icons.search_rounded,
          color: AppTheme.gold.withAlpha(180),
        ),
        suffixIcon: controller.text.isNotEmpty
            ? IconButton(
                icon: Icon(
                  Icons.close_rounded,
                  color: AppTheme.beige.withAlpha(140),
                ),
                onPressed: () {
                  controller.clear();
                  onClear?.call();
                },
              )
            : null,
        filled: true,
        fillColor: AppTheme.bgBlack,
        contentPadding: const EdgeInsets.symmetric(
          horizontal: 20,
          vertical: 16,
        ),
        border: OutlineInputBorder(
          borderRadius: BorderRadius.circular(16),
          borderSide: BorderSide(color: AppTheme.gold.withAlpha(50)),
        ),
        enabledBorder: OutlineInputBorder(
          borderRadius: BorderRadius.circular(16),
          borderSide: BorderSide(color: AppTheme.gold.withAlpha(50)),
        ),
        focusedBorder: OutlineInputBorder(
          borderRadius: BorderRadius.circular(16),
          borderSide: const BorderSide(color: AppTheme.gold, width: 1.5),
        ),
      ),
    );
  }
}
