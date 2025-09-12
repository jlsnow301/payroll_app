export function Denied() {
  return (
    <div className="absolute right-0 top-1/2 transform -translate-y-1/2 translate-x-4 pointer-events-none">
      <svg
        viewBox="0 0 220 70"
        className="w-56 h-20 -rotate-12 opacity-95 text-red-600 drop-shadow-lg"
        aria-hidden="true"
        xmlns="http://www.w3.org/2000/svg"
      >
        <defs>
          {/* rough displacement for border/text edges only (no blending with background) */}
          <filter
            id="rough"
            x="-10%"
            y="-10%"
            width="120%"
            height="120%"
          >
            <feTurbulence
              type="fractalNoise"
              baseFrequency="0.8"
              numOctaves="2"
            />
            <feDisplacementMap in="SourceGraphic" scale="3" />
          </filter>
        </defs>

        {/* rough dashed border (uses displacement filter but doesn't blend/mask page content) */}
        <g filter="url(#rough)" opacity="0.95">
          <rect
            x="6"
            y="6"
            width="208"
            height="58"
            rx="8"
            ry="8"
            stroke="currentColor"
            strokeWidth={8}
            fill="none"
            strokeDasharray="6 5"
            strokeLinecap="round"
            strokeLinejoin="round"
          />
        </g>

        {/* stacked text for rubber look (no interior mask or blending) */}
        <text
          x="110"
          y="44"
          textAnchor="middle"
          fontSize={30}
          fontWeight={800}
          fill="currentColor"
          stroke="rgba(255,255,255,0.6)"
          strokeWidth={2}
          paintOrder="stroke"
          style={{ mixBlendMode: "multiply" }}
          filter="url(#rough)"
        >
          DENIED
        </text>

        <text
          x="110"
          y="44"
          textAnchor="middle"
          fontSize={30}
          fontWeight={800}
          fill="none"
          stroke="currentColor"
          strokeWidth={4}
          opacity={0.28}
          filter="url(#rough)"
        >
          DENIED
        </text>
      </svg>
    </div>
  );
}
