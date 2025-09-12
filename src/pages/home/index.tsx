import { Button } from "@/components/ui/button.tsx";
import {
  Card,
  CardContent,
  CardDescription,
  CardFooter,
  CardHeader,
  CardTitle,
} from "@/components/ui/card.tsx";
import { Slider } from "@/components/ui/slider.tsx";
import {
  Tooltip,
  TooltipContent,
  TooltipTrigger,
} from "@/components/ui/tooltip.tsx";
import { PurpleBg, Stars } from "@/features/animated-bg/stars.tsx";
import { FileDropButton } from "@/features/file-drop/file-drop-button.tsx";
import { Page, usePrecision, useSimpleRouter } from "@/hooks.ts";
import { RefreshCcw } from "lucide-react";
import {
  useCatereaseMutation,
  useIntuitMutation,
  useSubmitMutation,
} from "./api.ts";
import { ErrorAlert } from "./error-alert.tsx";
import { ResultSection } from "./result-section.tsx";

export function HomePage() {
  const [_page, setPage] = useSimpleRouter();
  const [precision, setPrecision] = usePrecision();

  const catereaseMut = useCatereaseMutation();
  const intuitMut = useIntuitMutation();
  const submitMut = useSubmitMutation();

  const ready = catereaseMut.isSuccess && intuitMut.isSuccess &&
    submitMut.isIdle;

  /** Resets the state of submit once a new file is input */
  function handleFileInput(): void {
    if (!submitMut.isIdle) {
      submitMut.reset();
    }
  }

  function reset(): void {
    catereaseMut.reset();
    intuitMut.reset();
    submitMut.reset();
  }

  const errors: string[] = [];
  if (catereaseMut.isError) errors.push(`Caterease- ${catereaseMut.error}`);
  if (intuitMut.isError) errors.push(`Intuit- ${intuitMut.error}`);
  if (submitMut.isError) errors.push(`Process- ${submitMut.error}`);

  return (
    <PurpleBg>
      <Stars />
      <Card className="w-7/8 h-7/8 z-10 opacity-90 select-none">
        <CardHeader>
          <CardTitle className="text-xl flex justify-between h-8">
            <span>🗃 Payroll App</span>
            {!submitMut.isIdle && (
              <Button onClick={reset} size="sm" variant="ghost">
                <RefreshCcw /> Reset
              </Button>
            )}
          </CardTitle>
          <CardDescription className="opacity-100 text-md font-bold">
            This application takes two input files from caterease (the orders)
            and intuit (the employee hours). Select or drag the files onto their
            respective squares to begin.
          </CardDescription>
        </CardHeader>
        <CardContent className="flex-1 flex flex-col gap-4 justify-center items-center">
          <div className="flex w-full gap-8 justify-around">
            <FileDropButton
              mutation={catereaseMut}
              reset={handleFileInput}
              title="Caterease"
              tooltipContent="Requires an export from caterease- all orders in a given pay period."
            />
            <FileDropButton
              mutation={intuitMut}
              reset={handleFileInput}
              title="Intuit"
              tooltipContent="Requires an export from Intuit. Find this in the 'reports' section. Contains miles and a timesheet page."
            />
            <div className="flex flex-col gap-2 items-center justify-center">
              <Tooltip delayDuration={500}>
                <TooltipTrigger>
                  <span>
                    Precision: {precision} hour{precision > 1 && "s"}
                  </span>
                </TooltipTrigger>
                <TooltipContent>
                  <p>
                    Adjusts the leniency while matching order ready times with
                    clock in times.<br />More leniency may provide more matches,
                    but at the cost of accuracy.<br />Piggy back orders will
                    almost certainly be affected.
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
          </div>
          <div className="flex-1 w-3/4">
            {errors.length > 0 && (
              <ErrorAlert
                errors={errors}
                reset={reset}
              />
            )}
          </div>
        </CardContent>
        <CardFooter>
          <ResultSection
            mutation={submitMut}
            onSubmit={() => submitMut.mutate(precision)}
            onReview={() => setPage(Page.Review)}
            ready={ready}
          />
        </CardFooter>
      </Card>
    </PurpleBg>
  );
}
