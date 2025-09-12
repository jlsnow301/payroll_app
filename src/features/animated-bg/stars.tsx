import { PropsWithChildren, useMemo } from "react";

type Star = {
  id: number;
  x: number;
  y: number;
  size: number;
  animationDelay: number;
  animationDuration: number;
};

function generateStars(count: number): Star[] {
  return Array.from({ length: count }, (_, i) => {
    return {
      id: i,
      x: Math.random() * 100,
      y: Math.random() * 100,
      size: Math.random() * 3 + 1,
      animationDelay: Math.random() * 4,
      animationDuration: Math.random() * 3 + 2,
    } satisfies Star;
  });
}

export function PurpleBg(props: PropsWithChildren) {
  const { children } = props;

  return (
    <div className="
      flex flex-1 justify-center items-center
      bg-gradient-to-br from-purple-900 via-violet-800 to-indigo-800
      relative overflow-hidden">
      {children}
    </div>
  );
}

export function Stars() {
  const stars = useMemo(() => generateStars(150), []);

  return (
    <>
      {/* Animated Stars */}
      <div
        className="absolute inset-0 animate-pulse"
        style={{
          animation: "pulse 2s ease-in-out infinite, drift 20s linear infinite",
          animationName: "pulse, drift",
        }}
      >
        <style
          dangerouslySetInnerHTML={{
            __html: `
            @keyframes drift {
              from { transform: translate(0, 0); }
              to { transform: translate(100px, -100px); }
            }
          `,
          }}
        />
        {stars.map((star) => (
          <div
            key={star.id}
            className="absolute animate-pulse"
            style={{
              left: `${star.x}%`,
              top: `${star.y}%`,
              animationDelay: `${star.animationDelay}s`,
              animationDuration: `${star.animationDuration}s`,
            }}
          >
            <div
              className="bg-violet-300 rounded-full shadow-lg"
              style={{
                width: `${star.size}px`,
                height: `${star.size}px`,
                boxShadow: `0 0 ${star.size * 2}px rgba(196, 181, 253, 0.8)`,
              }}
            />
          </div>
        ))}
      </div>

      {/* Floating particles */}
      <div
        className="absolute inset-0"
        style={{
          animation: "drift 20s linear infinite",
        }}
      >
        {Array.from({ length: 30 }).map((_, i) => (
          <div
            key={`particle-${i}`}
            className="absolute w-1 h-1 bg-violet-400 rounded-full animate-bounce opacity-60"
            style={{
              left: `${Math.random() * 100}%`,
              top: `${Math.random() * 100}%`,
              animationDelay: `${Math.random() * 5}s`,
              animationDuration: `${Math.random() * 4 + 3}s`,
            }}
          />
        ))}
      </div>

      {/* Retro grid overlay */}
      <div
        className="absolute inset-0 opacity-10"
        style={{
          backgroundImage: `
            linear-gradient(rgba(196, 181, 253, 0.3) 1px, transparent 1px),
            linear-gradient(90deg, rgba(196, 181, 253, 0.3) 1px, transparent 1px)
          `,
          backgroundSize: "50px 50px",
          animation: "drift 20s linear infinite",
        }}
      />

      {/* Animated glow effects */}
      <div
        className="absolute inset-0"
        style={{
          animation: "drift 20s linear infinite",
        }}
      >
        {Array.from({ length: 8 }).map((_, i) => (
          <div
            key={`glow-${i}`}
            className="absolute rounded-full opacity-20 animate-ping"
            style={{
              left: `${Math.random() * 100}%`,
              top: `${Math.random() * 100}%`,
              width: `${Math.random() * 200 + 50}px`,
              height: `${Math.random() * 200 + 50}px`,
              background:
                "radial-gradient(circle, rgba(196, 181, 253, 0.4) 0%, transparent 70%)",
              animationDelay: `${Math.random() * 6}s`,
              animationDuration: `${Math.random() * 4 + 4}s`,
            }}
          />
        ))}
      </div>
    </>
  );
}
