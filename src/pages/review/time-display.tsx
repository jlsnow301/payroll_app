import { Threshold } from "@/features/time.ts";
import { ReactNode } from "react";

type Props = {
  color?: Threshold;
  icon: ReactNode;
  time: string;
  label: string;
};

export function TimeDisplay(props: Props) {
  const { color = Threshold.Gray, icon, time, label } = props;

  return (
    <div className="flex flex-col gap-2 text-center">
      {color == Threshold.Gray && <GrayTime icon={icon} time={time} />}
      {color == Threshold.Green && <GreenTime icon={icon} time={time} />}
      {color == Threshold.Yellow && <YellowTime icon={icon} time={time} />}
      {color == Threshold.Red && <RedTime icon={icon} time={time} />}
      <div className="text-xs text-gray-500">
        {label}
      </div>
    </div>
  );
}

type ColorProps = {
  icon: ReactNode;
  time: string;
};

// All this just to fool tailwind! psh!
function GreenTime(props: ColorProps) {
  const { icon, time } = props;

  return (
    <div className="flex items-center gap-2 text-sm text-green-500">
      {icon}
      <div className="text-xs text-green-300">
        {time}
      </div>
    </div>
  );
}

function YellowTime(props: ColorProps) {
  const { icon, time } = props;

  return (
    <div className="flex items-center gap-2 text-sm text-yellow-500">
      {icon}
      <div className="text-xs text-yellow-300">
        {time}
      </div>
    </div>
  );
}

function RedTime(props: ColorProps) {
  const { icon, time } = props;

  return (
    <div className="flex items-center gap-2 text-sm text-red-500">
      {icon}
      <div className="text-xs text-red-300">
        {time}
      </div>
    </div>
  );
}

function GrayTime(props: ColorProps) {
  const { icon, time } = props;

  return (
    <div className="flex items-center gap-2 text-sm text-gray-300">
      {icon}
      <div className="text-xs text-gray-300">
        {time}
      </div>
    </div>
  );
}
