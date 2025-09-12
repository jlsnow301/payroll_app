import { Button } from "@/components/ui/button.tsx";
import { PurpleBg } from "@/features/animated-bg/stars.tsx";
import { Page, usePrecision, useSimpleRouter } from "@/hooks.ts";
import { useVirtualizer } from "@tanstack/react-virtual";
import { Check, Undo, X } from "lucide-react";
import { useEffect, useMemo, useRef, useState } from "react";
import { useGetReview, useReviewMutation } from "./api.ts";
import { OrderCard } from "./order-card.tsx";

export function ReviewPage() {
  const [_page, setPage] = useSimpleRouter();
  const [precision] = usePrecision();

  const { data } = useGetReview(precision);
  const reviewMut = useReviewMutation();

  const withId = useMemo(() => {
    if (!data || data.rows.length === 0) {
      return [];
    }

    return data.rows.map((row, id) => ({
      ...row,
      id,
      approved: true,
    }));
  }, [data]);

  const [matches, setMatches] = useState(withId);

  useEffect(() => {
    if (withId.length > 0) setMatches(withId);
  }, [data]);

  function handleReject(id: number) {
    const newOrders = [...matches];

    const entry = {
      ...withId[id],
      hours: 0.0,
      miles: 0.0,
      suggestedIn: null,
      suggestedOut: null,
      approved: false,
    };

    newOrders[id] = entry;

    setMatches(newOrders);
  }

  function handleReset(id: number) {
    const newOrders = [...matches];

    newOrders[id] = {
      ...withId[id],
      approved: true,
    };

    setMatches(newOrders);
  }

  const parentRef = useRef<HTMLDivElement>(null);

  const rowVirtualizer = useVirtualizer({
    count: withId.length,
    getScrollElement: () => parentRef.current,
    estimateSize: () => 96,
  });

  return (
    <PurpleBg>
      <div className="flex h-full w-full flex-col gap-2 p-4 z-10 select-none">
        <div
          className="flex-1 overflow-auto"
          ref={parentRef}
        >
          <div
            className="relative flex flex-col items-center gap-3"
            style={{
              height: `${rowVirtualizer.getTotalSize()}px`,
            }}
          >
            {withId.filter((row) => row.suggestedIn).map((row) => {
              const corresponding = matches?.[row.id]?.approved;

              return (
                <div className="flex w-7/8 items-center gap-4" key={row.id}>
                  <OrderCard row={row} approved={corresponding} />
                  {corresponding
                    ? (
                      <Check onClick={() => handleReject(row.id)} />
                    )
                    : <X onClick={() => handleReset(row.id)} />}
                </div>
              );
            })}
          </div>
        </div>
        <div className="flex justify-between">
          <Button
            onClick={() => setPage(Page.Home)}
            className="w-36"
            variant="secondary"
          >
            <Undo /> Nevermind
          </Button>
          <span>Viewing {matches.length} Matches</span>
          <Button onClick={() => reviewMut.mutate(matches)} className="w-36">
            Submit
          </Button>
        </div>
      </div>
    </PurpleBg>
  );
}
