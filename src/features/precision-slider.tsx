import { Slider } from "@/components/ui/slider.tsx";
import {
  Tooltip,
  TooltipContent,
  TooltipTrigger,
} from "@/components/ui/tooltip.tsx";
import { Dispatch, SetStateAction } from "react";

type Props = {
  precision: number;
  setPrecision: Dispatch<SetStateAction<number>>;
};

export function PrecisionSlider(props: Props) {
  const { precision, setPrecision } = props;

  return (
    <div className="flex flex-col gap-2 items-center justify-center">
      <Tooltip delayDuration={500}>
        <TooltipTrigger>
          <span>
            Precision: {precision} hour{precision > 1 && "s"}
          </span>
        </TooltipTrigger>
        <TooltipContent>
          <p>
            Adjusts the leniency while matching order ready times with clock in
            times.<br />More leniency may provide more matches, but at the cost
            of accuracy.<br />Piggy back orders will almost certainly be
            affected.
          </p>
        </TooltipContent>
      </Tooltip>
      <Slider
        className="w-36"
        value={[precision]}
        max={5}
        step={1}
        min={1}
        onValueChange={(val) => setPrecision(val[0])}
      />
    </div>
  );
}
