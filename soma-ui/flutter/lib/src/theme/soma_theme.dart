import 'package:flutter/material.dart';
import 'soma_colors.dart';

class SomaTheme extends InheritedWidget {
  final SomaColors colors;
  final ThemeMode themeMode;

  const SomaTheme({
    super.key,
    required this.colors,
    required this.themeMode,
    required super.child,
  });

  /// Returns the current [SomaColors] from the nearest [SomaTheme] ancestor.
  static SomaColors of(BuildContext context) {
    final theme = context.dependOnInheritedWidgetOfExactType<SomaTheme>();
    assert(theme != null, 'No SomaTheme found in context. Wrap your app with SomaThemeProvider.');
    return theme!.colors;
  }

  /// Builds a [ThemeData] wired to the current soma colors.
  static ThemeData buildThemeData(BuildContext context) {
    final c = of(context);
    return ThemeData(
      fontFamily: 'Outfit',
      scaffoldBackgroundColor: c.background,
      colorScheme: ColorScheme(
        brightness: c == SomaColors.dark ? Brightness.dark : Brightness.light,
        primary: c.primary,
        onPrimary: c.primaryForeground,
        secondary: c.secondary,
        onSecondary: c.secondaryForeground,
        error: c.destructive,
        onError: c.destructiveForeground,
        surface: c.card,
        onSurface: c.cardForeground,
      ),
      textTheme: TextTheme(
        displayLarge: TextStyle(fontFamily: 'Rajdhani', fontWeight: FontWeight.w700, color: c.foreground),
        displayMedium: TextStyle(fontFamily: 'Rajdhani', fontWeight: FontWeight.w700, color: c.foreground),
        displaySmall: TextStyle(fontFamily: 'Rajdhani', fontWeight: FontWeight.w700, color: c.foreground),
        headlineLarge: TextStyle(fontFamily: 'Rajdhani', fontWeight: FontWeight.w600, color: c.foreground),
        headlineMedium: TextStyle(fontFamily: 'Rajdhani', fontWeight: FontWeight.w600, color: c.foreground),
        headlineSmall: TextStyle(fontFamily: 'Rajdhani', fontWeight: FontWeight.w600, color: c.foreground),
        titleLarge: TextStyle(fontFamily: 'Rajdhani', fontWeight: FontWeight.w600, color: c.foreground),
        titleMedium: TextStyle(fontFamily: 'Outfit', fontWeight: FontWeight.w500, color: c.foreground),
        titleSmall: TextStyle(fontFamily: 'Outfit', fontWeight: FontWeight.w500, color: c.foreground),
        bodyLarge: TextStyle(fontFamily: 'Outfit', color: c.foreground),
        bodyMedium: TextStyle(fontFamily: 'Outfit', color: c.foreground),
        bodySmall: TextStyle(fontFamily: 'Outfit', color: c.mutedForeground),
        labelLarge: TextStyle(fontFamily: 'Outfit', fontWeight: FontWeight.w500, color: c.foreground),
        labelMedium: TextStyle(fontFamily: 'Outfit', color: c.foreground),
        labelSmall: TextStyle(fontFamily: 'Outfit', color: c.mutedForeground),
      ),
      appBarTheme: AppBarTheme(
        backgroundColor: c.card,
        foregroundColor: c.foreground,
        elevation: 0,
        titleTextStyle: TextStyle(
          fontFamily: 'Rajdhani',
          fontWeight: FontWeight.w600,
          fontSize: 20,
          color: c.foreground,
        ),
      ),
      dividerColor: c.border,
      inputDecorationTheme: InputDecorationTheme(
        border: OutlineInputBorder(
          borderSide: BorderSide(color: c.input),
          borderRadius: BorderRadius.circular(6),
        ),
        focusedBorder: OutlineInputBorder(
          borderSide: BorderSide(color: c.ring, width: 2),
          borderRadius: BorderRadius.circular(6),
        ),
        hintStyle: TextStyle(color: c.mutedForeground),
        filled: true,
        fillColor: Colors.transparent,
      ),
    );
  }

  @override
  bool updateShouldNotify(SomaTheme oldWidget) =>
      oldWidget.colors != colors || oldWidget.themeMode != themeMode;
}

/// Wraps the widget tree with [SomaTheme], providing light/dark colors.
class SomaThemeProvider extends StatelessWidget {
  final ThemeMode themeMode;
  final Widget child;

  const SomaThemeProvider({
    super.key,
    this.themeMode = ThemeMode.dark,
    required this.child,
  });

  @override
  Widget build(BuildContext context) {
    final isDark = themeMode == ThemeMode.dark ||
        (themeMode == ThemeMode.system &&
            MediaQuery.platformBrightnessOf(context) == Brightness.dark);
    return SomaTheme(
      colors: isDark ? SomaColors.dark : SomaColors.light,
      themeMode: themeMode,
      child: child,
    );
  }
}
