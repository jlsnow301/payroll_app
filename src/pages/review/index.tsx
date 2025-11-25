import { ArrowLeft, ArrowRight, RotateCcw } from "lucide-react";
import { useMemo, useState } from "react";
import { Button } from "../../components/ui/button.tsx";
import { Stars } from "../../features/animated-bg/stars.tsx";
import { LootCard } from "../../features/loot-card/index.tsx";
import { StatsCard } from "../../features/loot-card/stats-card.tsx";

import { PurpleBg } from "../../features/animated-bg/purple-bg.tsx";
import { getStack } from "../../features/loot-card/weighted-random.ts";
import { useSwipe } from "../../features/swipe/index.ts";
import { Page, useSimpleRouter } from "../../hooks.ts";
import { useStatsData } from "./data.ts";

export function ReviewPage() {
  const [statsData, setStatsData] = useStatsData();
  const [_page, setPage] = useSimpleRouter();
  const [currentIndex, setCurrentIndex] = useState(0);
  const [touchStart, setTouchStart] = useState<number | null>(null);

  const stack = useMemo(() => getStack(statsData.length), [statsData.length]);

  function handleNext(): void {
    setCurrentIndex((prev) => (prev + 1) % statsData.length);
  }

  function handlePrev(): void {
    setCurrentIndex(
      (prev) => (prev - 1 + statsData.length) % statsData.length,
    );
  }

  const { ref } = useSwipe({
    onSwipe(direction) {
      if (direction === "left") handleNext();
      if (direction === "right") handlePrev();
    },
  });

  function handleKeyDown(evt: React.KeyboardEvent): void {
    if (evt.key === "ArrowRight") handleNext();
    if (evt.key === "ArrowLeft") handlePrev();
  }

  function handleTouchStart(evt: React.TouchEvent): void {
    setTouchStart(evt.touches[0].clientX);
  }

  function handleTouchEnd(evt: React.TouchEvent): void {
    if (touchStart === null) return;
    const touchEnd = evt.changedTouches[0].clientX;
    const diff = touchStart - touchEnd;

    if (Math.abs(diff) > 50) {
      if (diff > 0) handleNext();
      else handlePrev();
    }
    setTouchStart(null);
  }

  function handleReset(): void {
    setStatsData([]);
    setPage(Page.Home);
  }

  function getDirection(index: number): "left" | "right" | "center" | null {
    if (index === currentIndex) return "center";

    const prevIndex = (currentIndex - 1 + statsData.length) %
      statsData.length;
    const nextIndex = (currentIndex + 1) % statsData.length;

    if (index === prevIndex) return "left";
    if (index === nextIndex) return "right";

    // Hide all other cards
    return null;
  }

  if (!statsData || statsData.length === 0) {
    return (
      <PurpleBg>
        <Stars />
        <div className="relative z-10 flex flex-col items-center justify-center w-full h-full">
          <p className="text-white/70">No data available</p>
          <Button variant="secondary" onClick={handleReset} className="mt-4">
            <RotateCcw className="size-4 mr-2" />
            Go Back
          </Button>
        </div>
      </PurpleBg>
    );
  }

  return (
    <PurpleBg>
      <Stars />

      {/* Reset Button */}
      <Button
        variant="ghost"
        size="sm"
        onClick={handleReset}
        className={`
          absolute top-4 right-4 z-20
          transition-all duration-700 ease-out
          animate-in fade-in slide-in-from-top-4 delay-2500 fill-mode-backwards
        `}
      >
        <RotateCcw className="size-4" />
        Reset
      </Button>

      <div
        className="relative z-10 flex flex-col items-center justify-center w-full h-full select-none outline-none"
        tabIndex={0}
        onKeyDown={handleKeyDown}
        onTouchStart={handleTouchStart}
        onTouchEnd={handleTouchEnd}
      >
        {/* Title */}
        <div className="mb-8 text-center animate-in fade-in slide-in-from-top-8 duration-700 delay-300 fill-mode-backwards">
          <h1 className="text-4xl font-black text-white drop-shadow-lg tracking-wider">
            📊 STATS UNLOCKED 📊
          </h1>
          <p className="text-white/70 mt-2">
            Swipe or use arrow keys to browse
          </p>
        </div>

        {/* Cards container */}
        <div
          className="relative w-72 h-96 animate-in fade-in zoom-in-50 duration-700 delay-300 fill-mode-backwards"
          ref={ref}
        >
          {statsData.map((stat, index) => {
            const direction = getDirection(index);
            if (direction === null) return;

            // First card is always the general stats card
            if (stat.id === 1) {
              return (
                <StatsCard
                  key={stat.id}
                  stat={stat}
                  isActive={index === currentIndex}
                  direction={direction}
                />
              );
            }

            return (
              <LootCard
                key={stat.id}
                stat={stat}
                rarity={stack[index]}
                isActive={index === currentIndex}
                direction={direction}
              />
            );
          })}
        </div>

        {/* Navigation dots */}
        <div className="flex gap-3 mt-8 animate-in fade-in slide-in-from-bottom-8 duration-700 delay-500 fill-mode-backwards">
          {statsData.map((_, index) => (
            <button
              type="button"
              key={index}
              onClick={() => setCurrentIndex(index)}
              className={`
                w-3 h-3 rounded-full transition-all duration-300
                ${
                index === currentIndex
                  ? "bg-yellow-400 scale-125 shadow-lg shadow-yellow-400/50"
                  : "bg-white/40 hover:bg-white/60"
              }
              `}
            />
          ))}
        </div>

        {/* Navigation arrows */}
        <div className="flex gap-8 mt-6 animate-in fade-in duration-700 delay-700 fill-mode-backwards">
          <Button
            variant="secondary"
            onClick={handlePrev}
            className="hover:text-white hover:scale-110 transition-all"
          >
            <ArrowLeft className="mx-2" />
          </Button>
          <Button
            variant="secondary"
            onClick={handleNext}
            className="hover:text-white hover:scale-110 transition-all"
          >
            <ArrowRight className="mx-2" />
          </Button>
        </div>
      </div>
    </PurpleBg>
  );
}
