export function Stars() {
  return (
    <>
      {/* Animated Stars */}
      <div
        className="absolute animate-drift"
        style={{
          top: "-120px",
          left: "-120px",
          right: "-120px",
          bottom: "-120px",
        }}
      >
        {Array.from({ length: 150 }).map((_, i) => {
          const size = Math.random() * 3 + 1;

          return (
            <div
              key={`star-${i}`}
              className="absolute animate-pulse"
              style={{
                left: `${Math.random() * 100}%`,
                top: `${Math.random() * 100}%`,
                animationDelay: `${Math.random() * 4}s`,
                animationDuration: `${Math.random() * 3 + 2}s`,
              }}
            >
              <div
                className="bg-violet-300 rounded-full shadow-lg"
                style={{
                  width: `${size}px`,
                  height: `${size}px`,
                  boxShadow: `0 0 ${size * 2}px rgba(196, 181, 253, 0.8)`,
                }}
              />
            </div>
          );
        })}
      </div>

      {/* Floating particles */}
      <div
        className="absolute animate-drift"
        style={{
          top: "-120px",
          left: "-120px",
          right: "-120px",
          bottom: "-120px",
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
        className="absolute inset-0 opacity-10 animate-bg-drift"
        style={{
          backgroundImage: `
            linear-gradient(rgba(196, 181, 253, 0.3) 1px, transparent 1px),
            linear-gradient(90deg, rgba(196, 181, 253, 0.3) 1px, transparent 1px)
          `,
          backgroundSize: "50px 50px",
        }}
      />

      {/* Animated glow effects */}
      <div
        className="absolute animate-drift pointer-events-none"
        style={{
          top: "-120px",
          left: "-120px",
          right: "-120px",
          bottom: "-120px",
        }}
      >
        {Array.from({ length: 5 }).map((_, i) => (
          <div
            key={`glow-${i}`}
            className="absolute rounded-full opacity-20 animate-ping"
            style={{
              left: `${Math.random() * 100}%`,
              top: `${Math.random() * 100}%`,
              width: `${Math.random() * 200 + 50}px`,
              height: `${Math.random() * 200 + 50}px`,
              transform: "translate(-50%, -50%)",
              borderRadius: "50%",
              filter: "blur(28px)",
              willChange: "transform, opacity",
              background:
                "radial-gradient(circle at center, rgba(196,181,253,0.45) 0%, rgba(196,181,253,0.22) 25%, rgba(196,181,253,0.08) 45%, transparent 60%)",
              animationDelay: `${Math.random() * 6}s`,
              animationDuration: `${Math.random() * 4 + 4}s`,
            }}
          />
        ))}
      </div>
    </>
  );
}
