import 'package:flutter/material.dart';
import 'package:app/src/rust/api/simple.dart';
import 'package:app/theme.dart';

class TopMatchCard extends StatelessWidget {
  final TopMatch match;
  final VoidCallback? onTap;

  const TopMatchCard({super.key, required this.match, this.onTap});

  @override
  Widget build(BuildContext context) {
    return Material(
      color: Colors.transparent,
      child: InkWell(
        onTap: onTap,
        borderRadius: BorderRadius.circular(16),
        child: Container(
          margin: const EdgeInsets.symmetric(horizontal: 16, vertical: 6),
          decoration: BoxDecoration(
            borderRadius: BorderRadius.circular(16),
            gradient: LinearGradient(
              begin: Alignment.topLeft,
              end: Alignment.bottomRight,
              colors: [
                AppTheme.bgBlack.withAlpha(255),
                AppTheme.bgBlack.withAlpha(230),
              ],
            ),
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
            child: Row(
              children: [
                // Soccer ball icon
                Container(
                  padding: const EdgeInsets.all(10),
                  decoration: BoxDecoration(
                    shape: BoxShape.circle,
                    color: AppTheme.gold.withAlpha(25),
                  ),
                  child: const Icon(
                    Icons.sports_soccer,
                    color: AppTheme.gold,
                    size: 22,
                  ),
                ),
                const SizedBox(width: 14),
                // Match details
                Expanded(
                  child: Column(
                    crossAxisAlignment: CrossAxisAlignment.start,
                    children: [
                      Text(
                        match.teams,
                        style: const TextStyle(
                          color: AppTheme.beige,
                          fontWeight: FontWeight.w600,
                          fontSize: 15,
                        ),
                      ),
                      const SizedBox(height: 6),
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
                    ],
                  ),
                ),
                // Arrow
                Icon(
                  Icons.chevron_right_rounded,
                  color: AppTheme.gold.withAlpha(120),
                ),
              ],
            ),
          ),
        ),
      ),
    );
  }
}
