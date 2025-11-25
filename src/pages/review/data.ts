import { atom, useAtom } from "jotai";
import type { ProcessResult } from "../home/api.ts";

export type StatCard = {
  id: number;
  header: string;
  winner: string;
  details: string;
  icon?: string;
};

export const statsDataAtom = atom<StatCard[]>([]);

export function useStatsData() {
  return useAtom(statsDataAtom);
}

/**
 * Formats minutes into a human-readable string
 */
function formatMinutes(minutes: number): string {
  const absMinutes = Math.abs(minutes);
  if (absMinutes < 60) {
    return `${Math.round(absMinutes)} min`;
  }
  const hours = Math.floor(absMinutes / 60);
  const mins = Math.round(absMinutes % 60);
  return mins > 0 ? `${hours}h ${mins}m` : `${hours}h`;
}

/**
 * Generates stat cards from the process result
 */
export function generateStatsData(result: ProcessResult): StatCard[] {
  return [
    {
      id: 1,
      header: "GENERAL STATS",
      winner: "",
      details: [
        `${result.total} Total Orders`,
        `${result.expanded} Multiples Expanded`,
        `${result.matched} Matched`,
        `${result.skipped} Skipped/Takeout`,
      ].join("\n"),
    },
    {
      id: 2,
      header: "MOST UTILIZED",
      winner: result.top_used,
      details: `${result.top_used_count} deliveries`,
      icon: "🚚",
    },
    {
      id: 3,
      header: "MOST PUNCTUAL",
      winner: result.punctual,
      details: `Avg ${result.punctual_avg >= 0 ? "+" : ""}${
        formatMinutes(result.punctual_avg)
      } • ${result.punctual_late_count} late`,
      icon: "⏰",
    },
    {
      id: 4,
      header: "MOST LATE",
      winner: result.most_late,
      details: `${result.most_late_count} late clock-ins`,
      icon: "⌛",
    },
    {
      id: 5,
      header: "HIGHEST LATE %",
      winner: result.highest_late_percent_driver,
      details: `Late ${result.highest_late_percent.toFixed(1)}% of the time`,
      icon: "📉",
    },
    {
      id: 6,
      header: "LATEST CLOCK-IN",
      winner: result.latest_clock_in_driver,
      details: `${formatMinutes(result.latest_clock_in_diff_minutes)} late`,
      icon: "🦥",
    },
  ];
}
