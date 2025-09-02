import { Button } from "@/components/ui/button";
import {
  Card,
  CardContent,
  CardDescription,
  CardFooter,
  CardHeader,
  CardTitle,
} from "@/components/ui/card";
import { CircleAlert, CircleCheck } from "lucide-react";
import { useState } from "react";
import {
  useCatereaseMutation,
  useIntuitMutation,
  useSubmitMutation,
} from "./api";
import { green500, red500 } from "./constants";
import { FileDropButton } from "./file-drop-button";

export function FileDropCard() {
  const submitMut = useSubmitMutation();
  const [hasCaterease, setHasCaterease] = useState(false);
  const [hasIntuit, setHasIntuit] = useState(false);

  const ready = hasCaterease && hasIntuit;

  function handleSubmit(): void {
    submitMut.mutate();

    setHasCaterease(false);
    setHasIntuit(false);

    setTimeout(() => {
      submitMut.reset();
    }, 3000);
  }

  return (
    <Card className="w-7/8 h-7/8 z-10 opacity-90">
      <CardHeader>
        <CardTitle className="text-xl">Payroll App</CardTitle>
        <CardDescription className="opacity-100 text-md font-bold">
          This application takes two input files from caterease (the orders) and
          intuit (the employee hours). Drag the files onto their respective
          squares to begin.
        </CardDescription>
      </CardHeader>
      <CardContent className="h-full">
        <div className="flex gap-8 flex-1 justify-around items-center">
          <FileDropButton
            mutation={useCatereaseMutation}
            title="Caterease"
            state={hasCaterease}
            setter={setHasCaterease}
          />
          <FileDropButton
            mutation={useIntuitMutation}
            title="Intuit"
            state={hasIntuit}
            setter={setHasIntuit}
          />
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
