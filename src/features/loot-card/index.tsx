import {
  Card,
  CardContent,
  CardHeader,
  CardTitle,
} from "../../components/ui/card.tsx";
import type { StatCard } from "../../pages/review/data.ts";
import { glowColors, positionClasses, rarityColors } from "./constants.ts";
import { Rarity } from "./types.ts";

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
        className="absolute -inset-12 rounded-3xl animate-aura-pulse opacity-0 delay-600 fill-mode-forwards"
        style={{
          background:
            `radial-gradient(ellipse at center, ${colors.primary} 0%, ${colors.secondary} 30%, transparent 70%)`,
          filter: "blur(30px)",
        }}
      />

      {/* Mid-layer breathing glow */}
      <div
        className="absolute -inset-6 rounded-2xl animate-glow-breathe opacity-0 delay-600 fill-mode-forwards"
        style={{
          background:
            `radial-gradient(ellipse at center, ${colors.accent} 0%, ${colors.primary} 40%, transparent 70%)`,
          filter: "blur(22px)",
        }}
      />

      {/* Inner intense glow */}
      <div
        className="absolute -inset-3 rounded-xl animate-inner-glow opacity-0 delay-600 fill-mode-forwards"
        style={{
          background:
            `radial-gradient(ellipse at center, ${colors.accent} 0%, transparent 60%)`,
          filter: "blur(12px)",
        }}
      />

      {/* Shimmer sweep effect */}
      <div className="absolute inset-0 rounded-xl overflow-hidden">
        <div
          className="absolute inset-0 animate-shimmer-sweep opacity-0 delay-600 fill-mode-forwards"
          style={{
            background:
              `linear-gradient(105deg, transparent 40%, ${colors.accent} 50%, transparent 60%)`,
            filter: "blur(6px)",
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
  rarity: Rarity;
};

export function LootCard(props: LootProps) {
  const {
    stat,
    isActive,
    direction,
    rarity,
  } = props;

  return (
    <div
      className={`
        absolute transition-all duration-500 ease-out
        ${positionClasses[direction]}
        ${isActive ? "animate-card-pop [animation-delay:50ms]" : ""}
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

        {/* Rarity badge - corner ribbon style */}
        <div className="absolute -top-1 -right-1 overflow-hidden w-20 h-20 pointer-events-none">
          <div className="absolute top-4 -right-6 w-28 text-center rotate-45 bg-black/50 backdrop-blur-sm py-0.5 shadow-lg border-y border-white/10">
            <span className="text-[9px] font-bold tracking-[0.15em] text-white/90 uppercase">
              {direction === "right" ? "???" : rarity}
            </span>
          </div>
        </div>

        <CardHeader className="text-center pt-6 pb-2">
          <CardTitle
            className={`
              text-2xl font-black text-white drop-shadow-lg tracking-wide
              ${direction === "right" ? "blur-md select-none" : ""}
            `}
          >
            {direction === "right" ? "?????" : stat.header}
          </CardTitle>
        </CardHeader>

        <CardContent className="flex flex-col items-center justify-start pt-2 gap-4 text-center">
          {/* Winner avatar placeholder */}
          <div className="w-20 h-20 rounded-full bg-gradient-to-br from-white/40 to-white/10 border-4 border-white/50 flex items-center justify-center shadow-inner">
            <span className="text-3xl">
              {direction === "right" ? "❓" : stat.icon}
            </span>
          </div>

          {/* Winner name */}
          <div
            className={`space-y-1 ${
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
