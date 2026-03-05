import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:app/providers/settings_provider.dart';
import 'package:app/components/country_picker.dart';
import 'package:app/theme.dart';

class SettingsPage extends ConsumerStatefulWidget {
  const SettingsPage({super.key});

  @override
  ConsumerState<SettingsPage> createState() => _SettingsPageState();
}

class _SettingsPageState extends ConsumerState<SettingsPage> {
  late final TextEditingController _teamController;

  @override
  void initState() {
    super.initState();
    _teamController = TextEditingController();
    // Defer reading provider until after first build
    WidgetsBinding.instance.addPostFrameCallback((_) {
      final settings = ref.read(settingsProvider);
      _teamController.text = settings.favoriteTeam ?? '';
    });
  }

  @override
  void dispose() {
    _teamController.dispose();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    final settings = ref.watch(settingsProvider);

    return Scaffold(
      appBar: AppBar(title: const Text('⚙️ Settings')),
      body: ListView(
        padding: const EdgeInsets.all(20),
        children: [
          // Favorite Team Section
          _SectionHeader(title: 'Favorite Team'),
          const SizedBox(height: 12),
          TextField(
            controller: _teamController,
            style: const TextStyle(color: AppTheme.beige, fontSize: 16),
            cursorColor: AppTheme.gold,
            decoration: InputDecoration(
              hintText: 'e.g. Arsenal, PSG, Barcelona...',
              hintStyle: TextStyle(color: AppTheme.beige.withAlpha(100)),
              prefixIcon: Icon(
                Icons.favorite_rounded,
                color: AppTheme.rustOrange.withAlpha(180),
              ),
              filled: true,
              fillColor: AppTheme.bgBlack,
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
            onSubmitted: (value) {
              ref.read(settingsProvider.notifier).setFavoriteTeam(value);
              ScaffoldMessenger.of(context).showSnackBar(
                SnackBar(
                  content: Text(
                    value.trim().isEmpty
                        ? 'Favorite team cleared'
                        : 'Favorite team saved: ${value.trim()}',
                  ),
                  backgroundColor: AppTheme.gold.withAlpha(200),
                  behavior: SnackBarBehavior.floating,
                  shape: RoundedRectangleBorder(
                    borderRadius: BorderRadius.circular(10),
                  ),
                ),
              );
            },
          ),
          const SizedBox(height: 8),
          Text(
            'Press enter to save',
            style: TextStyle(
              color: AppTheme.beige.withAlpha(100),
              fontSize: 12,
            ),
          ),

          const SizedBox(height: 32),

          // Default Country Section
          _SectionHeader(title: 'Default Country'),
          const SizedBox(height: 12),
          CountryPicker(
            selected: settings.defaultCountry,
            onChanged: (country) {
              ref.read(settingsProvider.notifier).setDefaultCountry(country);
            },
          ),
          const SizedBox(height: 8),
          Text(
            'Used as the default provider when searching',
            style: TextStyle(
              color: AppTheme.beige.withAlpha(100),
              fontSize: 12,
            ),
          ),

          const SizedBox(height: 48),

          // App Info
          Center(
            child: Column(
              children: [
                const Icon(
                  Icons.sports_soccer_rounded,
                  color: AppTheme.gold,
                  size: 32,
                ),
                const SizedBox(height: 8),
                Text(
                  'Foot Info',
                  style: TextStyle(
                    color: AppTheme.gold.withAlpha(200),
                    fontSize: 16,
                    fontWeight: FontWeight.w600,
                  ),
                ),
                const SizedBox(height: 4),
                Text(
                  'Football match schedules & TV channels',
                  style: TextStyle(
                    color: AppTheme.beige.withAlpha(100),
                    fontSize: 12,
                  ),
                ),
              ],
            ),
          ),
        ],
      ),
    );
  }
}

class _SectionHeader extends StatelessWidget {
  final String title;

  const _SectionHeader({required this.title});

  @override
  Widget build(BuildContext context) {
    return Text(
      title,
      style: const TextStyle(
        color: AppTheme.gold,
        fontSize: 18,
        fontWeight: FontWeight.w600,
      ),
    );
  }
}
