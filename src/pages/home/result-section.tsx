import {
  CircleAlert,
  CircleCheck,
  RotateCcw,
  WandSparkles,
} from "lucide-react";
import { Button } from "../../components/ui/button.tsx";
import {
  Tooltip,
  TooltipContent,
  TooltipTrigger,
} from "../../components/ui/tooltip.tsx";
import { green500, red500 } from "../../constants.ts";
import { useSubmitMutation } from "./api.ts";

type Props = {
  mutation: ReturnType<typeof useSubmitMutation>;
  onSubmit: () => void;
  ready: boolean;
  reset: () => void;
  showReset: boolean;
};

export function ResultSection(props: Props) {
  const { mutation, ready, onSubmit, reset, showReset } = props;

  let submitIcon = <WandSparkles />;
  if (mutation.isSuccess) {
    submitIcon = <CircleCheck color={green500} />;
  } else if (mutation.isError) {
    submitIcon = <CircleAlert color={red500} />;
  }

  return (
    <div className="flex gap-2 items-center">
      {showReset && (
        <Button
          size="lg"
          variant="destructive"
          onClick={reset}
          className="w-36 transition-transform hover:scale-105 active:scale-95"
        >
          <RotateCcw /> Reset
        </Button>
      )}
      <Tooltip delayDuration={500}>
        <TooltipTrigger asChild>
          <Button
            disabled={!ready}
            onClick={onSubmit}
            className="w-36 transition-transform hover:scale-105 active:scale-95"
            size="lg"
          >
            {submitIcon} Auto
          </Button>
        </TooltipTrigger>
        <TooltipContent>
          {ready
            ? "Assign all matches within the precision"
            : "Both files must be uploaded and valid"}
        </TooltipContent>
      </Tooltip>
    </div>
  );
}
