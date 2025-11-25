import {
  Card,
  CardContent,
  CardHeader,
  CardTitle,
} from "../../components/ui/card.tsx";
import type { StatCard } from "../../pages/review/data.ts";
import { positionClasses, rarityColors } from "./constants.ts";

type Props = {
  stat: StatCard;
  isActive: boolean;
  direction: "left" | "right" | "center";
};

export function StatsCard(props: Props) {
  const { stat, isActive, direction } = props;

  const statsLines = stat.details.split("\n");

  return (
    <div
      className={`
        absolute transition-all duration-500 ease-out
        ${positionClasses[direction]}
        ${isActive ? "animate-card-pop [animation-delay:50ms]" : ""}
      `}
    >
      <Card
        className={`
          relative w-72 h-96 
          bg-gradient-to-br ${rarityColors.common}
          border-4 shadow-2xl
          transition-[filter,transform] duration-300
          overflow-hidden
          hover:scale-105
        `}
      >
        {/* Shine effect */}
        <div className="absolute inset-0 bg-gradient-to-br from-white/30 via-transparent to-transparent pointer-events-none" />

        <CardHeader className="text-center pt-8">
          <CardTitle
            className={`
              text-2xl font-black text-white drop-shadow-lg tracking-wide
            `}
          >
            {stat.header}
          </CardTitle>
        </CardHeader>

        <CardContent className="flex flex-col items-center justify-center flex-1 gap-4 text-center">
          <div className="space-y-3">
            {statsLines.map((line, index) => (
              <p
                key={index}
                className="text-lg text-white/90 font-medium"
              >
                {line}
              </p>
            ))}
          </div>
        </CardContent>

        {/* Bottom decoration */}
        <div className="absolute bottom-0 left-0 right-0 h-16 bg-gradient-to-t from-black/30 to-transparent" />
      </Card>
    </div>
  );
}
