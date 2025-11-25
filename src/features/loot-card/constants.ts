export const rarityColors = {
  legendary:
    "from-yellow-600 via-amber-500 to-yellow-600 border-yellow-400 shadow-yellow-500/50",
  epic:
    "from-purple-600 via-violet-500 to-purple-600 border-purple-400 shadow-purple-500/50",
  rare:
    "from-blue-600 via-cyan-500 to-blue-600 border-blue-400 shadow-blue-500/50",
  common:
    "from-gray-600 via-gray-500 to-gray-600 border-gray-400 shadow-gray-500/50",
} as const;

export const positionClasses = {
  left: "-translate-x-[120%] scale-75 opacity-30 rotate-[-8deg]",
  right: "translate-x-[120%] scale-75 opacity-30 rotate-[8deg]",
  center: "translate-x-0 scale-100 opacity-100 rotate-0",
} as const;

export const glowColors = {
  legendary: {
    primary: "rgba(255, 215, 0, 0.8)",
    secondary: "rgba(255, 170, 0, 0.6)",
    accent: "rgba(255, 240, 150, 0.9)",
  },
  epic: {
    primary: "rgba(167, 139, 250, 0.8)",
    secondary: "rgba(139, 92, 246, 0.6)",
    accent: "rgba(200, 180, 255, 0.9)",
  },
  rare: {
    primary: "rgba(96, 165, 250, 0.8)",
    secondary: "rgba(59, 130, 246, 0.6)",
    accent: "rgba(150, 200, 255, 0.9)",
  },
  common: {
    primary: "rgba(156, 163, 175, 0.6)",
    secondary: "rgba(107, 114, 128, 0.4)",
    accent: "rgba(209, 213, 219, 0.7)",
  },
} as const;
