import { Button } from "@/components/ui/button.tsx";
import { PurpleBg } from "@/features/animated-bg/stars.tsx";
import { Page, usePrecision, useSimpleRouter } from "@/hooks.ts";
import { useVirtualizer } from "@tanstack/react-virtual";
import { Undo } from "lucide-react";
import { useEffect, useMemo, useRef, useState } from "react";
import { PreparedRow, useGetReview, useReviewMutation } from "./api.ts";
import { OrderCard } from "./order-card.tsx";

type PreparedRowExt = {
  id: number;
  approved: boolean;
} & PreparedRow;

function addInitialValues(rows: PreparedRow[] | undefined): PreparedRowExt[] {
  if (!rows || rows.length === 0) {
    return [];
  }

  return rows.map((row, id) => {
    return {
      ...row,
      id,
      approved: true,
    };
  });
}

export function ReviewPage() {
  const [_page, setPage] = useSimpleRouter();
  const [precision] = usePrecision();

  const { data } = useGetReview(precision);
  const reviewMut = useReviewMutation();

  const withId = useMemo(() => addInitialValues(data?.rows), [data?.rows]);
  const [matches, setMatches] = useState(() => withId);

  useEffect(() => {
    setMatches(withId);
  }, [withId]);

  function handleClick(id: number) {
    const entry = matches[id];

    if (entry.approved) {
      handleReject(id);
    } else {
      handleReset(id);
    }
  }

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

  const filtered = withId.filter((row) => row.suggestedIn);

  const rowVirtualizer = useVirtualizer({
    count: filtered.length,
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
            {filtered.map((row) => {
              const corresponding = matches?.[row.id]?.approved;

              return (
                <div
                  className="w-7/8 hover:cursor-pointer"
                  onClick={handleClick.bind(null, row.id)}
                  key={row.id}
                >
                  <OrderCard row={row} approved={corresponding} />
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
            <Undo /> {reviewMut.isIdle ? "Nevermind" : "Return"}
          </Button>
          <div className="flex flex-col items-center justify-center">
            <span>Viewing {matches.length} Matches</span>
            <span className="text-xs text-muted-foreground">
              Click to reject
            </span>
          </div>
          <Button
            disabled={!reviewMut.isIdle}
            onClick={() => reviewMut.mutate(matches)}
            className="w-36"
          >
            Submit
          </Button>
        </div>
      </div>
    </PurpleBg>
  );
}
