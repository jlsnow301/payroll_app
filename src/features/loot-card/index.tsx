import { useEffect, useMemo, useState } from "react";
import {
  Card,
  CardContent,
  CardHeader,
  CardTitle,
} from "../../components/ui/card.tsx";
import { STATS_DATA } from "../../pages/review/data.ts";
import { glowColors, positionClasses, rarityColors } from "./constants.ts";

type StatCard = (typeof STATS_DATA)[number];
type Rarity = "legendary" | "epic" | "rare" | "common";

const COLORED_RARITIES: Rarity[] = ["legendary", "epic", "rare"];

function getRandomRarity(): Rarity {
  return COLORED_RARITIES[Math.floor(Math.random() * COLORED_RARITIES.length)];
}

type CardGlowProps = {
  rarity: Rarity;
};

function CardGlow(props: CardGlowProps) {
  const { rarity } = props;

  const colors = glowColors[rarity];

  return (
    <div
      className="absolute inset-0 pointer-events-none"
      style={{ zIndex: -1 }}
    >
      {/* Outer pulsing aura */}
      <div
        className="absolute -inset-8 rounded-3xl animate-aura-pulse opacity-0 delay-600 fill-mode-forwards"
        style={{
          background:
            `radial-gradient(ellipse at center, ${colors.primary} 0%, ${colors.secondary} 30%, transparent 70%)`,
          filter: "blur(20px)",
        }}
      />

      {/* Mid-layer breathing glow */}
      <div
        className="absolute -inset-4 rounded-2xl animate-glow-breathe opacity-0 delay-600 fill-mode-forwards"
        style={{
          background:
            `radial-gradient(ellipse at center, ${colors.accent} 0%, ${colors.primary} 40%, transparent 70%)`,
          filter: "blur(15px)",
        }}
      />

      {/* Inner intense glow */}
      <div
        className="absolute -inset-2 rounded-xl animate-inner-glow opacity-0 delay-600 fill-mode-forwards"
        style={{
          background:
            `radial-gradient(ellipse at center, ${colors.accent} 0%, transparent 60%)`,
          filter: "blur(8px)",
        }}
      />

      {/* Shimmer sweep effect */}
      <div className="absolute inset-0 rounded-xl overflow-hidden">
        <div
          className="absolute inset-0 animate-shimmer-sweep opacity-0 delay-600 fill-mode-forwards"
          style={{
            background:
              `linear-gradient(105deg, transparent 40%, ${colors.accent} 50%, transparent 60%)`,
            filter: "blur(4px)",
          }}
        />
      </div>
    </div>
  );
}

type LootProps = {
  stat: StatCard;
  isActive: boolean;
  direction: "left" | "right" | "center";
  cardIndex: number;
};

export function LootCard(props: LootProps) {
  const {
    stat,
    isActive,
    direction,
    cardIndex,
  } = props;

  const [hasAppeared, setHasAppeared] = useState(false);

  // First card is always common (uncolored), others get random rarity
  const rarity = useMemo<Rarity>(() => {
    if (cardIndex === 0) return "common";
    return getRandomRarity();
  }, [cardIndex]);

  useEffect(() => {
    if (isActive) {
      const timer = setTimeout(() => setHasAppeared(true), 50);
      return () => clearTimeout(timer);
    } else {
      setHasAppeared(false);
    }
  }, [isActive]);

  return (
    <div
      className={`
        absolute transition-all duration-500 ease-out
        ${positionClasses[direction]}
        ${hasAppeared && isActive ? "animate-card-pop" : ""}
      `}
    >
      {/* Glowing aura behind the card */}
      {isActive && rarity !== "common" && <CardGlow rarity={rarity} />}

      <Card
        className={`
          relative w-72 h-96 
          bg-gradient-to-br ${
          direction === "right"
            ? "from-gray-600 via-gray-500 to-gray-600 border-gray-400 shadow-gray-500/50"
            : rarityColors[rarity]
        }
          border-4 shadow-2xl
          transition-[filter,transform] duration-300
          overflow-hidden
          hover:scale-105
        `}
      >
        {/* Shine effect */}
        <div className="absolute inset-0 bg-gradient-to-br from-white/30 via-transparent to-transparent pointer-events-none" />

        <CardHeader className="text-center pt-8">
          <div className="text-xs font-bold tracking-[0.3em] text-white/80 uppercase mb-2">
            {direction === "right" ? "★ ??? ★" : `★ ${rarity} ★`}
          </div>
          <CardTitle
            className={`
              text-2xl font-black text-white drop-shadow-lg tracking-wide
              ${direction === "right" ? "blur-md select-none" : ""}
            `}
          >
            {direction === "right" ? "?????" : stat.header}
          </CardTitle>
        </CardHeader>

        <CardContent className="flex flex-col items-center justify-center flex-1 gap-6 text-center">
          {/* Winner avatar placeholder */}
          <div className="w-20 h-20 rounded-full bg-gradient-to-br from-white/40 to-white/10 border-4 border-white/50 flex items-center justify-center shadow-inner">
            <span className="text-3xl">
              {direction === "right" ? "❓" : "🏆"}
            </span>
          </div>

          {/* Winner name */}
          <div
            className={`space-y-2 ${
              direction === "right" ? "blur-md select-none" : ""
            }`}
          >
            <h3 className="text-2xl font-bold text-white drop-shadow-md">
              {direction === "right" ? "?????" : stat.winner}
            </h3>
            <p className="text-lg text-white/90 font-medium">
              {direction === "right" ? "???" : stat.details}
            </p>
          </div>
        </CardContent>

        {/* Bottom decoration */}
        <div className="absolute bottom-0 left-0 right-0 h-16 bg-gradient-to-t from-black/30 to-transparent" />
      </Card>
    </div>
  );
}
