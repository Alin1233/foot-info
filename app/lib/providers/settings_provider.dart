import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:shared_preferences/shared_preferences.dart';
import 'package:app/src/rust/api/simple.dart';

const _kFavoriteTeamKey = 'favorite_team';
const _kDefaultCountryKey = 'default_country';

/// Holds user settings: favorite team and default country.
class SettingsState {
  final String? favoriteTeam;
  final Country defaultCountry;

  const SettingsState({this.favoriteTeam, this.defaultCountry = Country.uk});

  SettingsState copyWith({
    String? favoriteTeam,
    Country? defaultCountry,
    bool clearFavorite = false,
  }) {
    return SettingsState(
      favoriteTeam: clearFavorite ? null : (favoriteTeam ?? this.favoriteTeam),
      defaultCountry: defaultCountry ?? this.defaultCountry,
    );
  }
}

class SettingsNotifier extends Notifier<SettingsState> {
  @override
  SettingsState build() {
    _load();
    return const SettingsState();
  }

  Future<void> _load() async {
    final prefs = await SharedPreferences.getInstance();
    final team = prefs.getString(_kFavoriteTeamKey);
    final countryIndex = prefs.getInt(_kDefaultCountryKey) ?? 0;
    state = SettingsState(
      favoriteTeam: team,
      defaultCountry: Country.values[countryIndex],
    );
  }

  Future<void> setFavoriteTeam(String? team) async {
    final prefs = await SharedPreferences.getInstance();
    if (team == null || team.trim().isEmpty) {
      await prefs.remove(_kFavoriteTeamKey);
      state = state.copyWith(clearFavorite: true);
    } else {
      await prefs.setString(_kFavoriteTeamKey, team.trim());
      state = state.copyWith(favoriteTeam: team.trim());
    }
  }

  Future<void> setDefaultCountry(Country country) async {
    final prefs = await SharedPreferences.getInstance();
    await prefs.setInt(_kDefaultCountryKey, country.index);
    state = state.copyWith(defaultCountry: country);
  }
}

final settingsProvider = NotifierProvider<SettingsNotifier, SettingsState>(
  SettingsNotifier.new,
);
