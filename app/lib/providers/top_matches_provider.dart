import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:app/src/rust/api/simple.dart';

/// Fetches top matches from LiveSoccerTV via the Rust FFI bridge.
final topMatchesProvider = FutureProvider<List<TopMatch>>((ref) async {
  return await fetchTopMatches();
});
