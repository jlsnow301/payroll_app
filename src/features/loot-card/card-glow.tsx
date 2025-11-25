import { glowColors } from "./constants.ts";
import { Rarity } from "./types.ts";

type CardGlowProps = {
  rarity: Rarity;
};

export function CardGlow(props: CardGlowProps) {
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
