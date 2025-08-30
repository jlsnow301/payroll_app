import { useMutation } from "@tanstack/react-query";
import { invoke } from "@tauri-apps/api/core";

export function useCatereaseMutation() {
  return useMutation({
    mutationFn: (filePath: string) => invoke("caterease_input", { filePath }),
  });
}

export function useIntuitMutation() {
  return useMutation({
    mutationFn: (filePath: string) => invoke("intuit_input", { filePath }),
  });
}

export function useSubmitMutation() {
  return useMutation({
    mutationFn: () => invoke("submit"),
  });
}
