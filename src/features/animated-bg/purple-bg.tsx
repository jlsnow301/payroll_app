import { PropsWithChildren } from "react";
import { Page, useSimpleRouter } from "../../hooks.ts";

export function PurpleBg(props: PropsWithChildren) {
  const { children } = props;
  const [page] = useSimpleRouter();
  const darken = page === Page.Review;

  return (
    <div className="flex flex-1 justify-center items-center bg-gradient-to-br from-purple-900 via-violet-800 to-indigo-800 relative overflow-hidden">
      {darken && (
        <div className="absolute inset-0 bg-black/60 pointer-events-none" />
      )}
      {children}
    </div>
  );
}
