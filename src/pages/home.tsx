import { StarBg } from "../features/animated-bg/stars";
import { FileDropCard } from "../features/file-drop/card";

export function FileDropPage() {
  return (
    <div className="
      flex flex-1 justify-center items-center
      bg-gradient-to-br from-purple-900 via-violet-800 to-indigo-800
      relative overflow-hidden">
      <StarBg />
      <FileDropCard />
    </div>
  );
}
