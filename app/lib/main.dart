import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:app/src/rust/api/simple.dart';
import 'package:app/src/rust/frb_generated.dart';
import 'package:app/theme.dart';

// Riverpod Provider for fetching top matches
final topMatchesProvider = FutureProvider<List<TopMatch>>((ref) async {
  return await fetchTopMatches();
});

Future<void> main() async {
  await RustLib.init();
  runApp(const ProviderScope(child: MyApp()));
}

class MyApp extends StatelessWidget {
  const MyApp({super.key});

  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      title: 'Foot Info',
      theme: AppTheme.darkTheme,
      home: const TopMatchesScreen(),
    );
  }
}

class TopMatchesScreen extends ConsumerWidget {
  const TopMatchesScreen({super.key});

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final matchesAsync = ref.watch(topMatchesProvider);

    return Scaffold(
      appBar: AppBar(
        title: const Text('Top Matches'),
        actions: [
          IconButton(
            icon: const Icon(Icons.refresh),
            onPressed: () {
              // Invalidate the provider to trigger a re-fetch
              ref.invalidate(topMatchesProvider);
            },
          ),
        ],
      ),
      body: matchesAsync.when(
        loading: () => const Center(
          child: CircularProgressIndicator(color: AppTheme.gold),
        ),
        error: (error, stack) => Center(
          child: Text(
            'Error: $error',
            style: const TextStyle(color: AppTheme.rustOrange),
          ),
        ),
        data: (matches) {
          if (matches.isEmpty) {
            return const Center(child: Text('No top matches found.'));
          }

          return ListView.builder(
            itemCount: matches.length,
            itemBuilder: (context, index) {
              final topMatch = matches[index];
              return Card(
                margin: const EdgeInsets.symmetric(horizontal: 16, vertical: 8),
                child: ListTile(
                  title: Text(
                    topMatch.teams,
                    style: const TextStyle(fontWeight: FontWeight.bold),
                  ),
                  subtitle: Text(
                    '${topMatch.date} @ ${topMatch.matchUrl ?? ""}',
                  ),
                  trailing: const Icon(
                    Icons.sports_soccer,
                    color: AppTheme.gold,
                  ),
                ),
              );
            },
          );
        },
      ),
    );
  }
}
