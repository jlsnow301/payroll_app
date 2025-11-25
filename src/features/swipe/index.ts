import { useEffect, useRef } from "react";

type Point = {
  x: number;
  y: number;
};

type Props = {
  /** Callback fn for which direction was swiped */
  onSwipe: (direction: "left" | "right") => void;
  /** Minimum distance in pixels for a swipe to be detected */
  threshold?: number;
};

/**
 * Swipe detection hook for both mouse and touch events.
 */
export function useSwipe(props: Props) {
  const { onSwipe, threshold = 50 } = props;

  const ref = useRef<HTMLDivElement>(null);
  const startPoint = useRef<Point | null>(null);

  useEffect(() => {
    const element = ref.current;
    if (!element) return;

    // Mouse-based swipe (for desktop environments like Tauri)
    function handleMouseDown(event: MouseEvent): void {
      // only left button
      if (event.button !== 0) return;
      startPoint.current = { x: event.clientX, y: event.clientY };
    }

    function handleMouseMove(event: MouseEvent): void {
      if (!startPoint.current) return;
      // optional: prevent text selection / native drag
      event.preventDefault();
    }

    function handleMouseUp(event: MouseEvent): void {
      if (!startPoint.current) return;
      const endPoint = { x: event.clientX, y: event.clientY };
      const deltaX = endPoint.x - startPoint.current.x;
      if (Math.abs(deltaX) > threshold) {
        const direction = deltaX > 0 ? "right" : "left";
        onSwipe(direction);
      }
      startPoint.current = null;
    }

    // Touch-based swipe (for touch-enabled devices)
    function handleTouchStart(event: TouchEvent): void {
      const t = event.touches[0];
      startPoint.current = { x: t.clientX, y: t.clientY };
    }

    function handleTouchMove(event: TouchEvent): void {
      if (!startPoint.current) return;
      // prevent native scrolling while swiping horizontally
      // Only prevent if horizontal movement is dominant could be improved later
      event.preventDefault();
    }

    function handleTouchEnd(event: TouchEvent): void {
      if (!startPoint.current) return;
      const t = event.changedTouches[0];
      const endPoint = { x: t.clientX, y: t.clientY };
      const deltaX = endPoint.x - startPoint.current.x;
      if (Math.abs(deltaX) > threshold) {
        const direction = deltaX > 0 ? "right" : "left";
        onSwipe(direction);
      }
      startPoint.current = null;
    }

    // Mouse events
    element.addEventListener("mousedown", handleMouseDown);
    globalThis.addEventListener("mousemove", handleMouseMove);
    globalThis.addEventListener("mouseup", handleMouseUp);

    // Touch events
    element.addEventListener("touchstart", handleTouchStart, {
      passive: false,
    });
    element.addEventListener("touchmove", handleTouchMove, { passive: false });
    element.addEventListener("touchend", handleTouchEnd);

    return () => {
      // Mouse events
      element.removeEventListener("mousedown", handleMouseDown);
      globalThis.removeEventListener("mousemove", handleMouseMove);
      globalThis.removeEventListener("mouseup", handleMouseUp);

      // Touch events
      element.removeEventListener(
        "touchstart",
        handleTouchStart as EventListener,
      );
      element.removeEventListener(
        "touchmove",
        handleTouchMove as EventListener,
      );
      element.removeEventListener("touchend", handleTouchEnd as EventListener);
    };
  }, [onSwipe, threshold]);

  return { ref };
}
