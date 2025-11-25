import { sample } from "es-toolkit/array";
import { ArrowLeft, ArrowRight, RotateCcw } from "lucide-react";
import { useEffect, useState } from "react";
import { Button } from "../../components/ui/button.tsx";
import { PurpleBg, Stars } from "../../features/animated-bg/stars.tsx";
import { LootCard } from "../../features/loot-card/index.tsx";
import { Page, useSimpleRouter, useSubmissionResult } from "../../hooks.ts";
import { STATS_DATA } from "./data.ts";

function _getStack(): string[] {
  const rarity = ["legendary", "epic", "rare", "common"];

  const stack: string[] = [];
  for (let i = 0; i < 5; i++) {
    stack.push(sample(rarity));
  }

  return stack;
}

export function ReviewPage() {
  const [_submissionResult, setSubmissionResult] = useSubmissionResult();
  const [_page, setPage] = useSimpleRouter();
  const [currentIndex, setCurrentIndex] = useState(0);
  const [isRevealed, setIsRevealed] = useState(false);
  const [touchStart, setTouchStart] = useState<number | null>(null);

  useEffect(() => {
    // Trigger the reveal animation after a short delay
    const timer = setTimeout(() => setIsRevealed(true), 300);
    return () => clearTimeout(timer);
  }, []);

  function handleNext(): void {
    setCurrentIndex((prev) => (prev + 1) % STATS_DATA.length);
  }

  function handlePrev(): void {
    setCurrentIndex(
      (prev) => (prev - 1 + STATS_DATA.length) % STATS_DATA.length,
    );
  }

  function handleKeyDown(e: React.KeyboardEvent): void {
    if (e.key === "ArrowRight") handleNext();
    if (e.key === "ArrowLeft") handlePrev();
  }

  function handleTouchStart(e: React.TouchEvent) {
    setTouchStart(e.touches[0].clientX);
  }

  function handleTouchEnd(e: React.TouchEvent): void {
    if (touchStart === null) return;
    const touchEnd = e.changedTouches[0].clientX;
    const diff = touchStart - touchEnd;

    if (Math.abs(diff) > 50) {
      if (diff > 0) handleNext();
      else handlePrev();
    }
    setTouchStart(null);
  }

  function handleReset(): void {
    setSubmissionResult(undefined);
    setPage(Page.Home);
  }

  function getDirection(index: number): "left" | "right" | "center" | null {
    if (index === currentIndex) return "center";

    const prevIndex = (currentIndex - 1 + STATS_DATA.length) %
      STATS_DATA.length;
    const nextIndex = (currentIndex + 1) % STATS_DATA.length;

    if (index === prevIndex) return "left";
    if (index === nextIndex) return "right";

    // Hide all other cards
    return null;
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
        <div
          className={`
            mb-8 text-center transition-all duration-700 ease-out
            ${
            isRevealed
              ? "opacity-100 translate-y-0"
              : "opacity-0 -translate-y-8"
          }
          `}
        >
          <h1 className="text-4xl font-black text-white drop-shadow-lg tracking-wider">
            STATS UNLOCKED
          </h1>
          <p className="text-white/70 mt-2">
            Swipe or use arrow keys to browse
          </p>
        </div>

        {/* Cards container */}
        <div
          className={`
            relative w-72 h-96 
            transition-all duration-700 ease-out
            ${isRevealed ? "opacity-100 scale-100" : "opacity-0 scale-50"}
          `}
        >
          {STATS_DATA.map((stat, index) => {
            const direction = getDirection(index);
            if (direction === null) return;
            return (
              <LootCard
                key={stat.id}
                stat={stat}
                isActive={index === currentIndex}
                direction={direction}
                cardIndex={index}
              />
            );
          })}
        </div>

        {/* Navigation dots */}
        <div
          className={`
            flex gap-3 mt-8 transition-all duration-700 delay-300
            ${
            isRevealed ? "opacity-100 translate-y-0" : "opacity-0 translate-y-8"
          }
          `}
        >
          {STATS_DATA.map((_, index) => (
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
        <div
          className={`
            flex gap-8 mt-6 transition-all duration-700 delay-500
            ${isRevealed ? "opacity-100" : "opacity-0"}
          `}
        >
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
