import 'package:app/src/rust/api/simple.dart';

/// Display info for each Country provider
class CountryInfo {
  final Country country;
  final String flag;
  final String label;

  const CountryInfo({
    required this.country,
    required this.flag,
    required this.label,
  });

  static const List<CountryInfo> all = [
    CountryInfo(country: Country.uk, flag: '🇬🇧', label: 'UK'),
    CountryInfo(country: Country.us, flag: '🇺🇸', label: 'US'),
    CountryInfo(country: Country.fr, flag: '🇫🇷', label: 'FR'),
  ];

  static CountryInfo fromCountry(Country country) {
    return all.firstWhere((info) => info.country == country);
  }
}
