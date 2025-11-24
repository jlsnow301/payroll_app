import { RefreshCcw } from "lucide-react";
import { Suspense, useEffect } from "react";
import { Button } from "../../components/ui/button.tsx";
import {
  Card,
  CardContent,
  CardDescription,
  CardFooter,
  CardHeader,
  CardTitle,
} from "../../components/ui/card.tsx";
import { AboutDialog } from "../../features/about.tsx";
import { PurpleBg, Stars } from "../../features/animated-bg/stars.tsx";
import { FileDropButton } from "../../features/file-drop/file-drop-button.tsx";
import { FileDropDialog } from "../../features/file-drop/file-drop-dialog.tsx";
import { PrecisionSlider } from "../../features/precision-slider.tsx";
import {
  Page,
  usePrecision,
  useSimpleRouter,
  useSubmissionResult,
} from "../../hooks.ts";
import {
  useCatereaseMutation,
  useGetHeaders,
  useIntuitMutation,
  useSubmitMutation,
} from "./api.ts";
import { ErrorAlert } from "./error-alert.tsx";
import { ResultSection } from "./result-section.tsx";

export function HomePage() {
  const [_page, setPage] = useSimpleRouter();
  const [precision, setPrecision] = usePrecision();
  const [_results, setResults] = useSubmissionResult();

  const expectedHeaders = useGetHeaders();

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

  useEffect(() => {
    if (submitMut.isSuccess) {
      setResults(submitMut.data);
      setPage(Page.Review);
    }
  }, [submitMut.isSuccess]);

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
            <AboutDialog />
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
          <Suspense fallback={<div>Loading...</div>}>
            <div className="flex w-full gap-8 justify-around">
              <div className="flex gap-2">
                <FileDropButton
                  mutation={catereaseMut}
                  reset={handleFileInput}
                  title="Caterease"
                />
                <FileDropDialog
                  title="Caterease File Drop"
                  description="Requires an export from Caterease. All orders in a given pay period."
                  headers={expectedHeaders.data.caterease}
                />
              </div>
              <div className="flex gap-2">
                <FileDropButton
                  mutation={intuitMut}
                  reset={handleFileInput}
                  title="Intuit"
                />
                <FileDropDialog
                  title="Intuit File Drop"
                  description="Requires an export from Intuit.
                Find this under reports - tracking - mileage.
                It must contain a sheet labeled 'Timesheets'"
                  headers={expectedHeaders.data.intuit}
                />
              </div>
              <PrecisionSlider
                precision={precision}
                setPrecision={setPrecision}
              />
            </div>
            <div className="flex-1 w-3/4">
              {errors.length > 0 && (
                <ErrorAlert
                  errors={errors}
                  reset={reset}
                />
              )}
            </div>
          </Suspense>
        </CardContent>
        <CardFooter>
          <ResultSection
            mutation={submitMut}
            onSubmit={() => submitMut.mutate(precision)}
            ready={ready}
          />
        </CardFooter>
      </Card>
    </PurpleBg>
  );
}
