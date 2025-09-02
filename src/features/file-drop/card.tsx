import { Button } from "@/components/ui/button";
import {
  Card,
  CardContent,
  CardDescription,
  CardFooter,
  CardHeader,
  CardTitle,
} from "@/components/ui/card";
import { AlertCircleIcon, CircleAlert, CircleCheck } from "lucide-react";
import {
  useCatereaseMutation,
  useIntuitMutation,
  useSubmitMutation,
} from "./api";
import { green500, red500 } from "./constants";
import { FileDropButton } from "./file-drop-button";
import { Alert, AlertDescription, AlertTitle } from "@/components/ui/alert";

export function FileDropCard() {
  const catereaseMut = useCatereaseMutation();
  const intuitMut = useIntuitMutation();

  const submitMut = useSubmitMutation();

  const ready = catereaseMut.isSuccess && intuitMut.isSuccess;

  function handleSubmit(): void {
    submitMut.mutate();

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
      <CardContent className="h-full">
        <div className="flex flex-1 flex-col gap-4 justify-around items-center">
          <div className="flex gap-8 justify-around items-center">
            <FileDropButton
              mutation={catereaseMut}
              title="Caterease"
            />
            <FileDropButton
              mutation={intuitMut}
              title="Intuit"
            />
          </div>
          <div className="flex-1 w-3/4">
            {errors.length > 0 && (
              <ErrorAlert
                errors={errors}
                reset={resetError}
              />
            )}
          </div>
        </div>
      </CardContent>
      <CardFooter className="flex-row-reverse gap-2">
        <Button
          disabled={!ready}
          onClick={handleSubmit}
          className="w-36"
          size="lg"
        >
          {submitMut.isSuccess ? "Done" : ready ? "Submit" : "Waiting"}
        </Button>
        {submitMut.isSuccess && (
          <CircleCheck
            color={green500}
            className="animate-vanishing"
          />
        )}
        {submitMut.isError && (
          <CircleAlert
            color={red500}
            className="fade-in"
          />
        )}
        {submitMut.isError && <CircleAlert color={red500} />}
      </CardFooter>
    </Card>
  );
}

type ErrorAlertProps = {
  errors: string[];
  reset: () => void;
};

function ErrorAlert(props: ErrorAlertProps) {
  const { errors, reset } = props;

  return (
    <Alert
      variant="destructive"
      className="text-lg"
    >
      <AlertCircleIcon />
      <AlertTitle>Error detected</AlertTitle>
      <AlertDescription className="flex">
        <div className="flex flex-1 justify-between">
          <ul className="list-disc">
            {errors.map((msg) => <li>{msg}</li>)}
          </ul>
          <Button onClick={reset} size="sm" variant="outline">
            Reset
          </Button>
        </div>
      </AlertDescription>
    </Alert>
  );
}
