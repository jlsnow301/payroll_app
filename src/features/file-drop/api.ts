import { useMutation } from "@tanstack/react-query";
import { invoke } from "@tauri-apps/api/core";

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
  missing: number;
  total: number;
};

export function useSubmitMutation() {
  return useMutation({
    mutationFn: (precision: number) =>
      invoke<ProcessResult>("submit", { precision }),
  });
}
