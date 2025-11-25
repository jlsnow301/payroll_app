import { atom, useAtom } from "jotai";

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
