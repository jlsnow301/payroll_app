import { atom, useAtom } from "jotai";
import { ProcessResult } from "./pages/home/api.ts";

export enum Page {
  Home,
  Review,
}

const pageAtom = atom(Page.Home);

/** Pages! */
export function useSimpleRouter() {
  return useAtom(pageAtom);
}

const precisionAtom = atom(1);

/** Hours in either direction that will count as a matched time entry */
export function usePrecision() {
  return useAtom(precisionAtom);
}

export const submissionResultAtom = atom<ProcessResult | undefined>();

export function useSubmissionResult() {
  return useAtom(submissionResultAtom);
}
