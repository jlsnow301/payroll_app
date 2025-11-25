import { useMutation, useSuspenseQuery } from "@tanstack/react-query";
import { invoke } from "@tauri-apps/api/core";

type ExpectedHeaders = {
  caterease: string[];
  intuit: string[];
};

export function useGetHeaders() {
  return useSuspenseQuery({
    queryKey: ["headers"],
    queryFn: () => invoke<ExpectedHeaders>("get_headers"),
  });
}

export function useCatereaseMutation() {
  return useMutation({
    mutationFn: (filePath: string) =>
      invoke<string>("caterease_input", { filePath }),
  });
}

export function useIntuitMutation() {
  return useMutation({
    mutationFn: (filePath: string) =>
      invoke<string>("intuit_input", { filePath }),
  });
}

/// See stats/types.rs
export type ProcessResult = {
  expanded: number;
  matched: number;
  skipped: number;
  total: number;
  top_used: string;
  top_used_count: number;
  punctual: string;
  punctual_avg: number;
  punctual_late_count: number;
  most_late: string;
  most_late_count: number;
  highest_late_percent_driver: string;
  highest_late_percent: number;
  latest_clock_in_driver: string;
  latest_clock_in_diff_minutes: number;
};

export function useSubmitMutation() {
  return useMutation({
    mutationFn: (precision: number) =>
      invoke<ProcessResult>("submit", { precision }),
  });
}
