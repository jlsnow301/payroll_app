import { Rarity } from "./types.ts";

export type WeightedItem = {
  item: Rarity;
  weight: number;
};

export function weightedRandomPick(
  arr: WeightedItem[],
): string {
  let totalWeight = 0;
  for (let i = 0; i < arr.length; i++) {
    totalWeight += arr[i].weight;
  }

  let random = Math.random() * totalWeight;

  for (let i = 0; i < arr.length; i++) {
    random -= arr[i].weight;
    if (random <= 0) {
      return arr[i].item;
    }
  }

  return arr[arr.length - 1].item;
}

const rarities: WeightedItem[] = [
  { item: "legendary", weight: 1 },
  { item: "epic", weight: 3 },
  { item: "rare", weight: 5 },
  { item: "common", weight: 7 },
];

export function getStack(length: number = 5): Rarity[] {
  const stack: Rarity[] = ["common"];

  for (let i = 1; i < length; i++) {
    const sampledRarity = weightedRandomPick(rarities) as Rarity;
    stack.push(sampledRarity);
  }

  return stack;
}
