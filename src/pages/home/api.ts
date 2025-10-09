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

export type ProcessResult = {
  expanded: number;
  matched: number;
  skipped: number;
  total: number;
};

export function useSubmitMutation() {
  return useMutation({
    mutationFn: (precision: number) =>
      invoke<ProcessResult>("submit", { precision }),
  });
}
