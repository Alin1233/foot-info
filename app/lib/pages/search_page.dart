import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:app/providers/search_provider.dart';
import 'package:app/providers/settings_provider.dart';
import 'package:app/components/match_card.dart';
import 'package:app/components/country_picker.dart';
import 'package:app/components/search_bar.dart';
import 'package:app/components/loading_indicator.dart';
import 'package:app/components/error_display.dart';
import 'package:app/theme.dart';

class SearchPage extends ConsumerStatefulWidget {
  const SearchPage({super.key});

  @override
  ConsumerState<SearchPage> createState() => _SearchPageState();
}

class _SearchPageState extends ConsumerState<SearchPage> {
  late final TextEditingController _controller;

  @override
  void initState() {
    super.initState();
    _controller = TextEditingController();
  }

  @override
  void dispose() {
    _controller.dispose();
    super.dispose();
  }

  void _onSearch() {
    final query = _controller.text.trim();
    if (query.isEmpty) return;
    ref.read(searchProvider.notifier).setQuery(query);
    ref.read(searchProvider.notifier).search();
    FocusScope.of(context).unfocus();
  }

  @override
  Widget build(BuildContext context) {
    final state = ref.watch(searchProvider);

    // Sync search country when settings default country changes
    ref.listen(settingsProvider.select((s) => s.defaultCountry), (prev, next) {
      if (prev != next) {
        ref.read(searchProvider.notifier).setCountry(next);
      }
    });

    // Sync the text controller when the provider query changes
    // (e.g. when navigated from TopMatchesPage with a pre-filled team)
    if (_controller.text != state.query) {
      _controller.text = state.query;
      _controller.selection = TextSelection.fromPosition(
        TextPosition(offset: state.query.length),
      );
    }

    return Scaffold(
      appBar: AppBar(title: const Text('🔍 Search')),
      body: Column(
        children: [
          // Country picker
          Padding(
            padding: const EdgeInsets.fromLTRB(16, 12, 16, 8),
            child: CountryPicker(
              selected: state.selectedCountry,
              onChanged: (country) {
                ref.read(searchProvider.notifier).setCountry(country);
                // Re-search with new country if there's an active query
                if (state.query.isNotEmpty) {
                  ref.read(searchProvider.notifier).search();
                }
              },
            ),
          ),
          // Search bar
          Padding(
            padding: const EdgeInsets.symmetric(horizontal: 16, vertical: 8),
            child: AppSearchBar(
              controller: _controller,
              onSubmitted: _onSearch,
              onClear: () {
                ref.read(searchProvider.notifier).clearResults();
              },
            ),
          ),
          // Favorite team quick-fill
          _buildFavoriteChip(ref),
          const SizedBox(height: 4),
          // Results area
          Expanded(child: _buildResults(state)),
        ],
      ),
    );
  }

  Widget _buildFavoriteChip(WidgetRef ref) {
    final favoriteTeam = ref.watch(settingsProvider).favoriteTeam;
    if (favoriteTeam == null || favoriteTeam.isEmpty) {
      return const SizedBox.shrink();
    }

    return Padding(
      padding: const EdgeInsets.symmetric(horizontal: 16),
      child: Align(
        alignment: Alignment.centerLeft,
        child: ActionChip(
          avatar: Icon(
            Icons.favorite_rounded,
            size: 16,
            color: AppTheme.rustOrange.withAlpha(200),
          ),
          label: Text(favoriteTeam),
          labelStyle: TextStyle(
            color: AppTheme.beige.withAlpha(200),
            fontSize: 13,
          ),
          backgroundColor: AppTheme.rustOrange.withAlpha(25),
          side: BorderSide(color: AppTheme.rustOrange.withAlpha(60)),
          shape: RoundedRectangleBorder(
            borderRadius: BorderRadius.circular(20),
          ),
          onPressed: () {
            _controller.text = favoriteTeam;
            ref.read(searchProvider.notifier).setQuery(favoriteTeam);
            ref.read(searchProvider.notifier).search();
            FocusScope.of(context).unfocus();
          },
        ),
      ),
    );
  }

  Widget _buildResults(SearchState state) {
    if (state.isLoading) {
      return const LoadingIndicator(message: 'Searching...');
    }

    if (state.error != null) {
      return ErrorDisplay(message: state.error!, onRetry: _onSearch);
    }

    if (state.results.isEmpty && state.query.isNotEmpty) {
      return Center(
        child: Column(
          mainAxisSize: MainAxisSize.min,
          children: [
            Icon(
              Icons.search_off_rounded,
              size: 56,
              color: AppTheme.beige.withAlpha(80),
            ),
            const SizedBox(height: 12),
            Text(
              'No matches found for "${state.query}"',
              style: TextStyle(
                color: AppTheme.beige.withAlpha(160),
                fontSize: 15,
              ),
            ),
          ],
        ),
      );
    }

    if (state.results.isEmpty) {
      return Center(
        child: Column(
          mainAxisSize: MainAxisSize.min,
          children: [
            Icon(
              Icons.sports_soccer_rounded,
              size: 56,
              color: AppTheme.beige.withAlpha(60),
            ),
            const SizedBox(height: 12),
            Text(
              'Search for a team to see upcoming matches',
              style: TextStyle(
                color: AppTheme.beige.withAlpha(120),
                fontSize: 15,
              ),
            ),
          ],
        ),
      );
    }

    return ListView.builder(
      padding: const EdgeInsets.only(top: 8, bottom: 24),
      itemCount: state.results.length,
      itemBuilder: (context, index) {
        return MatchCard(match: state.results[index]);
      },
    );
  }
}
