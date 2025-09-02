import { Button } from "@/components/ui/button";
import { CircleAlert, CircleCheck } from "lucide-react";
import { green500, red500 } from "./constants";
import { UseMutationResult } from "@tanstack/react-query";
import { ProcessResult } from "./api";
import {
  Tooltip,
  TooltipContent,
  TooltipTrigger,
} from "@/components/ui/tooltip";
import {
  Dialog,
  DialogContent,
  DialogHeader,
  DialogTitle,
  DialogTrigger,
} from "@/components/ui/dialog";

type Props = {
  mutation: UseMutationResult<ProcessResult, Error, number, unknown>;
  ready: boolean;
  onSubmit: () => void;
};

export function ResultSection(props: Props) {
  const { mutation, ready, onSubmit } = props;

  return (
    <div className="flex flex-1 gap-2 justify-between">
      <div>
        {mutation.isSuccess && <ViewStats {...mutation.data} />}
      </div>
      <div>
        {mutation.isSuccess && (
          <CircleCheck className="animate-vanishing" color={green500} />
        )}
        {mutation.isError && (
          <CircleAlert className="animate-vanishing" color={red500} />
        )}
        <Tooltip>
          <TooltipTrigger>
            <Button
              disabled={!ready}
              onClick={onSubmit}
              className="w-36"
              size="lg"
            >
              {mutation.isSuccess ? "Done" : ready ? "Submit" : "Waiting"}
            </Button>
          </TooltipTrigger>
          <TooltipContent>
            {!ready && "Both files must be uploaded and valid"}
          </TooltipContent>
        </Tooltip>
      </div>
    </div>
  );
}

function ViewStats(props: ProcessResult) {
  const { matched, total, missing, expanded } = props;

  const accuracy = (missing / total * 100).toFixed(2);

  return (
    <Dialog>
      <DialogTrigger asChild>
        <Button variant="outline">
          View Stats
        </Button>
      </DialogTrigger>
      <DialogHeader>
        <DialogTitle>Processing Results</DialogTitle>
      </DialogHeader>
      <DialogContent>
        Overall, the process produced {total} orders with {matched}{" "}
        timesheet matches. Given the sensitivity, it had
        {accuracy}% matching accuracy ({missing}{" "}
        missing). Orders with multiple assignees were expanded to produce{" "}
        {expanded} rows, highlighted in yellow.
      </DialogContent>
    </Dialog>
  );
}
