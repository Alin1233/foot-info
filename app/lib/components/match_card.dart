import 'package:flutter/material.dart';
import 'package:app/src/rust/api/simple.dart';
import 'package:app/theme.dart';

class MatchCard extends StatelessWidget {
  final Match match;

  const MatchCard({super.key, required this.match});

  @override
  Widget build(BuildContext context) {
    return Container(
      margin: const EdgeInsets.symmetric(horizontal: 16, vertical: 6),
      decoration: BoxDecoration(
        borderRadius: BorderRadius.circular(16),
        color: AppTheme.bgBlack,
        border: Border.all(color: AppTheme.gold.withAlpha(40), width: 1),
        boxShadow: [
          BoxShadow(
            color: Colors.black.withAlpha(60),
            blurRadius: 8,
            offset: const Offset(0, 2),
          ),
        ],
      ),
      child: Padding(
        padding: const EdgeInsets.all(16),
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            // Teams
            Text(
              match.teams,
              style: const TextStyle(
                color: AppTheme.beige,
                fontWeight: FontWeight.w600,
                fontSize: 16,
              ),
            ),
            const SizedBox(height: 4),
            // Competition
            Text(
              match.competition,
              style: TextStyle(
                color: AppTheme.gold.withAlpha(200),
                fontSize: 13,
                fontWeight: FontWeight.w500,
              ),
            ),
            const SizedBox(height: 10),
            // Date & Time row
            Row(
              children: [
                Icon(
                  Icons.calendar_today_rounded,
                  size: 13,
                  color: AppTheme.beige.withAlpha(140),
                ),
                const SizedBox(width: 5),
                Text(
                  match.date,
                  style: TextStyle(
                    color: AppTheme.beige.withAlpha(160),
                    fontSize: 13,
                  ),
                ),
                const SizedBox(width: 14),
                Icon(
                  Icons.access_time_rounded,
                  size: 13,
                  color: AppTheme.beige.withAlpha(140),
                ),
                const SizedBox(width: 5),
                Text(
                  match.time,
                  style: TextStyle(
                    color: AppTheme.beige.withAlpha(160),
                    fontSize: 13,
                  ),
                ),
              ],
            ),
            // Channels
            if (match.channels.isNotEmpty) ...[
              const SizedBox(height: 12),
              Wrap(
                spacing: 6,
                runSpacing: 6,
                children: match.channels.map((channel) {
                  return Container(
                    padding: const EdgeInsets.symmetric(
                      horizontal: 10,
                      vertical: 5,
                    ),
                    decoration: BoxDecoration(
                      borderRadius: BorderRadius.circular(20),
                      color: AppTheme.rustOrange.withAlpha(30),
                      border: Border.all(
                        color: AppTheme.rustOrange.withAlpha(80),
                        width: 1,
                      ),
                    ),
                    child: Text(
                      channel,
                      style: const TextStyle(
                        color: AppTheme.rustOrange,
                        fontSize: 12,
                        fontWeight: FontWeight.w500,
                      ),
                    ),
                  );
                }).toList(),
              ),
            ],
          ],
        ),
      ),
    );
  }
}
