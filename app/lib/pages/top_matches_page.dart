import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:go_router/go_router.dart';
import 'package:app/providers/top_matches_provider.dart';
import 'package:app/providers/search_provider.dart';
import 'package:app/components/top_match_card.dart';
import 'package:app/components/loading_indicator.dart';
import 'package:app/components/error_display.dart';
import 'package:app/theme.dart';

class TopMatchesPage extends ConsumerWidget {
  const TopMatchesPage({super.key});

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final matchesAsync = ref.watch(topMatchesProvider);

    return Scaffold(
      appBar: AppBar(
        title: const Text('⚽ Top Matches'),
        actions: [
          IconButton(
            icon: const Icon(Icons.refresh_rounded),
            tooltip: 'Refresh',
            onPressed: () => ref.invalidate(topMatchesProvider),
          ),
        ],
      ),
      body: matchesAsync.when(
        loading: () =>
            const LoadingIndicator(message: 'Loading top matches...'),
        error: (error, _) => ErrorDisplay(
          message: 'Failed to load top matches.\n$error',
          onRetry: () => ref.invalidate(topMatchesProvider),
        ),
        data: (matches) {
          if (matches.isEmpty) {
            return Center(
              child: Column(
                mainAxisSize: MainAxisSize.min,
                children: [
                  Icon(
                    Icons.sports_soccer_rounded,
                    size: 64,
                    color: AppTheme.beige.withAlpha(80),
                  ),
                  const SizedBox(height: 16),
                  Text(
                    'No top matches found',
                    style: TextStyle(
                      color: AppTheme.beige.withAlpha(160),
                      fontSize: 16,
                    ),
                  ),
                ],
              ),
            );
          }

          return RefreshIndicator(
            color: AppTheme.gold,
            backgroundColor: AppTheme.bgBlack,
            onRefresh: () async => ref.invalidate(topMatchesProvider),
            child: ListView.builder(
              padding: const EdgeInsets.only(top: 8, bottom: 24),
              itemCount: matches.length,
              itemBuilder: (context, index) {
                return TopMatchCard(
                  match: matches[index],
                  onTap: () {
                    // Extract first team name (same as TUI: split by " - ")
                    final teamName = matches[index].teams
                        .split(' - ')
                        .first
                        .trim();

                    // Pre-fill search and trigger it
                    final notifier = ref.read(searchProvider.notifier);
                    notifier.setQuery(teamName);
                    notifier.search();

                    // Navigate to Search tab (index 1)
                    context.go('/search');
                  },
                );
              },
            ),
          );
        },
      ),
    );
  }
}
