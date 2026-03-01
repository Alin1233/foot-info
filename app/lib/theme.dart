import 'package:flutter/material.dart';

class AppTheme {
  // Original Ratatui Rust Colors
  // pub const BG_BLACK: Color = Color::Rgb(27, 26, 23); // #1B1A17
  // pub const GOLD: Color = Color::Rgb(240, 165, 0); // #F0A500
  // pub const RUST_ORANGE: Color = Color::Rgb(228, 88, 38); // #E45826
  // pub const BEIGE: Color = Color::Rgb(230, 213, 184); // #E6D5B8

  static const Color bgBlack = Color(0xFF1B1A17);
  static const Color gold = Color(0xFFF0A500);
  static const Color rustOrange = Color(0xFFE45826);
  static const Color beige = Color(0xFFE6D5B8);

  static ThemeData get darkTheme {
    return ThemeData(
      useMaterial3: true,
      scaffoldBackgroundColor: bgBlack,
      colorScheme: const ColorScheme.dark(
        primary: gold,
        secondary: rustOrange,
        surface: bgBlack,
        onSurface: beige,
        onPrimary: bgBlack,
        onSecondary: beige,
      ),
      appBarTheme: const AppBarTheme(
        backgroundColor: bgBlack,
        foregroundColor: gold,
        elevation: 0,
        centerTitle: true,
      ),
      textTheme: const TextTheme(
        bodyLarge: TextStyle(color: beige),
        bodyMedium: TextStyle(color: beige),
        titleLarge: TextStyle(color: gold, fontWeight: FontWeight.bold),
        titleMedium: TextStyle(color: gold),
      ),
      cardTheme: CardThemeData(
        color: bgBlack,
        elevation: 2,
        shape: RoundedRectangleBorder(
          borderRadius: BorderRadius.circular(12),
          side: BorderSide(color: gold.withAlpha(50), width: 1),
        ),
      ),
      listTileTheme: const ListTileThemeData(textColor: beige, iconColor: gold),
    );
  }
}
