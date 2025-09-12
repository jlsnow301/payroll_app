import { useMutation, useQuery } from "@tanstack/react-query";
import { invoke } from "@tauri-apps/api/core";

type Order = {
  client: string;
  count: number;
  date: number;
  datetime: string;
  description: string;
  employee: string;
  event: string;
  expanded: boolean;
  grat: number;
  origin: string;
  ready: number;
  total: number;
};

export type PreparedRow = {
  order: Order;
  hours: number | null;
  miles: number | null;
  suggestedIn: string | null;
  suggestedOut: string | null;
};

type ProcessResult = {
  rows: PreparedRow[];
  matched: number;
  skipped: number;
};

export function useGetReview(precision: number) {
  return useQuery({
    queryFn: () => invoke<ProcessResult>("manual_review", { precision }),
    queryKey: ["manualReview"],
  });
}

export function useReviewMutation() {
  return useMutation({
    mutationFn: (rows: PreparedRow[]) =>
      invoke<string>("manual_input", { rows }),
  });
}
