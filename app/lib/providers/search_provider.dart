import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:app/src/rust/api/simple.dart';

/// Holds the current search state: query, selected country, and results.
class SearchState {
  final String query;
  final Country selectedCountry;
  final List<Match> results;
  final bool isLoading;
  final String? error;

  const SearchState({
    this.query = '',
    this.selectedCountry = Country.uk,
    this.results = const [],
    this.isLoading = false,
    this.error,
  });

  SearchState copyWith({
    String? query,
    Country? selectedCountry,
    List<Match>? results,
    bool? isLoading,
    String? error,
  }) {
    return SearchState(
      query: query ?? this.query,
      selectedCountry: selectedCountry ?? this.selectedCountry,
      results: results ?? this.results,
      isLoading: isLoading ?? this.isLoading,
      error: error,
    );
  }
}

class SearchNotifier extends Notifier<SearchState> {
  @override
  SearchState build() => const SearchState();

  void setQuery(String query) {
    state = state.copyWith(query: query);
  }

  void setCountry(Country country) {
    state = state.copyWith(selectedCountry: country, results: [], error: null);
  }

  Future<void> search() async {
    final query = state.query.trim();
    if (query.isEmpty) return;

    state = state.copyWith(isLoading: true, error: null);

    try {
      final matches = await searchTeam(
        team: query,
        country: state.selectedCountry,
      );
      state = state.copyWith(results: matches, isLoading: false);
    } catch (e) {
      state = state.copyWith(
        isLoading: false,
        error: e.toString(),
        results: [],
      );
    }
  }

  void clearResults() {
    state = state.copyWith(query: '', results: [], error: null);
  }
}

final searchProvider = NotifierProvider<SearchNotifier, SearchState>(
  SearchNotifier.new,
);
