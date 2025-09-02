import {
  Card,
  CardContent,
  CardDescription,
  CardFooter,
  CardHeader,
  CardTitle,
} from "@/components/ui/card";
import {
  useCatereaseMutation,
  useIntuitMutation,
  useSubmitMutation,
} from "./api";
import { FileDropButton } from "./file-drop-button";
import { ResultSection } from "./result-section";
import { ErrorAlert } from "./error-alert";
import { useState } from "react";
import { Slider } from "@/components/ui/slider";
import {
  Tooltip,
  TooltipContent,
  TooltipTrigger,
} from "@/components/ui/tooltip";

export function FileDropCard() {
  const [precision, setPrecision] = useState(1);

  const catereaseMut = useCatereaseMutation();
  const intuitMut = useIntuitMutation();
  const submitMut = useSubmitMutation();

  const ready = catereaseMut.isSuccess && intuitMut.isSuccess;

  function handleSubmit(): void {
    submitMut.mutate(precision);

    catereaseMut.reset();
    intuitMut.reset();

    setTimeout(() => {
      submitMut.reset();
    }, 3000);
  }

  function resetError(): void {
    catereaseMut.reset();
    intuitMut.reset();
    submitMut.reset();
  }

  let errors: string[] = [];
  if (catereaseMut.isError) errors.push(`Caterease- ${catereaseMut.error}`);
  if (intuitMut.isError) errors.push(`Intuit- ${intuitMut.error}`);
  if (submitMut.isError) errors.push(`Process- ${submitMut.error}`);

  return (
    <Card className="w-7/8 h-7/8 z-10 opacity-90">
      <CardHeader>
        <CardTitle className="text-xl">🗃 Payroll App</CardTitle>
        <CardDescription className="opacity-100 text-md font-bold">
          This application takes two input files from caterease (the orders) and
          intuit (the employee hours). Drag the files onto their respective
          squares to begin.
        </CardDescription>
      </CardHeader>
      <CardContent className="flex-1 flex flex-col gap-4 justify-center items-center">
        <div className="flex w-full gap-8 justify-around">
          <FileDropButton
            mutation={catereaseMut}
            title="Caterease"
            tooltipContent="Requires an export from caterease- all orders in a given pay period"
          />
          <FileDropButton
            mutation={intuitMut}
            title="Intuit"
            tooltipContent="Requires an export from Intuit. Sidebar -> Reports -> Timesheets"
          />
          <div className="flex flex-col gap-2 items-center justify-center">
            <Tooltip>
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
              reset={resetError}
            />
          )}
        </div>
      </CardContent>
      <CardFooter>
        <ResultSection
          mutation={submitMut}
          onSubmit={handleSubmit}
          ready={ready}
        />
      </CardFooter>
    </Card>
  );
}
