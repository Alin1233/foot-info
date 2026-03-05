import 'package:flutter/material.dart';
import 'package:google_fonts/google_fonts.dart';

class AppTheme {
  // Brand colors from the TUI
  static const Color bgBlack = Color(0xFF1B1A17);
  static const Color gold = Color(0xFFF0A500);
  static const Color rustOrange = Color(0xFFE45826);
  static const Color beige = Color(0xFFE6D5B8);

  // Surface variations
  static const Color surfaceLight = Color(0xFF252420);
  static const Color surfaceDark = Color(0xFF141310);

  static ThemeData get darkTheme {
    final textTheme = GoogleFonts.interTextTheme(ThemeData.dark().textTheme);

    return ThemeData(
      useMaterial3: true,
      scaffoldBackgroundColor: surfaceLight,
      colorScheme: const ColorScheme.dark(
        primary: gold,
        secondary: rustOrange,
        surface: surfaceLight,
        onSurface: beige,
        onPrimary: bgBlack,
        onSecondary: beige,
      ),
      textTheme: textTheme.copyWith(
        bodyLarge: textTheme.bodyLarge?.copyWith(color: beige),
        bodyMedium: textTheme.bodyMedium?.copyWith(color: beige),
        titleLarge: textTheme.titleLarge?.copyWith(
          color: gold,
          fontWeight: FontWeight.bold,
        ),
        titleMedium: textTheme.titleMedium?.copyWith(color: gold),
        labelLarge: textTheme.labelLarge?.copyWith(color: beige),
      ),
      appBarTheme: AppBarTheme(
        backgroundColor: surfaceLight,
        foregroundColor: gold,
        elevation: 0,
        centerTitle: true,
        titleTextStyle: GoogleFonts.inter(
          fontSize: 20,
          fontWeight: FontWeight.w700,
          color: gold,
        ),
      ),
      cardTheme: CardThemeData(
        color: bgBlack,
        elevation: 2,
        shape: RoundedRectangleBorder(
          borderRadius: BorderRadius.circular(16),
          side: BorderSide(color: gold.withAlpha(40), width: 1),
        ),
      ),
      listTileTheme: const ListTileThemeData(textColor: beige, iconColor: gold),
      snackBarTheme: SnackBarThemeData(
        backgroundColor: gold.withAlpha(220),
        contentTextStyle: const TextStyle(color: bgBlack),
        behavior: SnackBarBehavior.floating,
        shape: RoundedRectangleBorder(borderRadius: BorderRadius.circular(10)),
      ),
      navigationBarTheme: NavigationBarThemeData(
        labelTextStyle: WidgetStateProperty.resolveWith((states) {
          if (states.contains(WidgetState.selected)) {
            return GoogleFonts.inter(
              fontSize: 12,
              fontWeight: FontWeight.w600,
              color: gold,
            );
          }
          return GoogleFonts.inter(fontSize: 12, color: beige.withAlpha(120));
        }),
        iconTheme: WidgetStateProperty.resolveWith((states) {
          if (states.contains(WidgetState.selected)) {
            return const IconThemeData(color: gold);
          }
          return IconThemeData(color: beige.withAlpha(120));
        }),
      ),
    );
  }
}
