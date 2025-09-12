import { Button } from "@/components/ui/button";
import {
  Dialog,
  DialogContent,
  DialogHeader,
  DialogTitle,
  DialogTrigger,
} from "@/components/ui/dialog";
import {
  Tooltip,
  TooltipContent,
  TooltipTrigger,
} from "@/components/ui/tooltip";
import { green500, red500 } from "@/constants";
import { UseMutationResult } from "@tanstack/react-query";
import {
  CircleAlert,
  CircleCheck,
  ListChecks,
  WandSparkles,
} from "lucide-react";
import { ProcessResult } from "./api";

type Props = {
  mutation: UseMutationResult<ProcessResult, Error, number, unknown>;
  ready: boolean;
  onReview: () => void;
  onSubmit: () => void;
};

export function ResultSection(props: Props) {
  const { mutation, ready, onReview, onSubmit } = props;

  let submitIcon = <WandSparkles />;
  if (mutation.isSuccess) {
    submitIcon = <CircleCheck color={green500} />;
  } else if (mutation.isError) {
    submitIcon = <CircleAlert color={red500} />;
  }

  return (
    <div className="flex flex-1 gap-2 justify-between items-center">
      <div>
        {mutation.isSuccess && <ViewStats {...mutation.data} />}
      </div>
      <div className="flex gap-2 items-center">
        <Tooltip delayDuration={500}>
          <TooltipTrigger asChild>
            <Button
              disabled={!ready}
              onClick={onReview}
              className="w-36"
              size="lg"
            >
              <ListChecks /> Manual
            </Button>
          </TooltipTrigger>
          <TooltipContent>
            {ready
              ? "Manually approve time suggestions"
              : "Both files must be uploaded and valid"}
          </TooltipContent>
        </Tooltip>
        <Tooltip delayDuration={500}>
          <TooltipTrigger asChild>
            <Button
              disabled={!ready}
              onClick={onSubmit}
              className="w-36"
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
    </div>
  );
}

function ViewStats(props: ProcessResult) {
  const { matched = 0, skipped = 0, total = 1, expanded = 0 } = props;

  const valid = total - skipped;
  const missing = valid - matched;
  const accuracy = (matched / valid * 100).toFixed(2);

  return (
    <Dialog>
      <DialogTrigger asChild>
        <Button size="lg" variant="outline">
          View Stats
        </Button>
      </DialogTrigger>
      <DialogContent>
        <DialogHeader>
          <DialogTitle>Processing Results</DialogTitle>
        </DialogHeader>
        Overall, the application wrote {total} orders with {matched}{" "}
        timesheet matches. Given the sensitivity, it had{" "}
        {accuracy}% accuracy ({`${missing} missing, ${skipped} skipped`}).
        Orders with multiple assignees were expanded to produce {expanded}{" "}
        rows, highlighted in yellow.
      </DialogContent>
    </Dialog>
  );
}
