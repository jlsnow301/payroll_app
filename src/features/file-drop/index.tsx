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
import { Button } from "@/components/ui/button";
import { CircleAlert, CircleCheck } from "lucide-react";
import { useState } from "react";
import { green500, red500 } from "./constants";

export function FileDropPage() {
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
    <div className="flex-1 flex items-center justify-center">
      <Card className="w-3/4 h-3/4">
        <CardHeader>
          <CardTitle>Payroll App</CardTitle>
          <CardDescription>
            This application takes two input files from caterease (the orders)
            and intuit (the employee hours).
          </CardDescription>
        </CardHeader>
        <CardContent className="h-full">
          <div className="flex gap-8 flex-1 justify-around items-center">
            <FileDropButton
              mutation={useCatereaseMutation}
              title="Caterease"
              setter={setHasCaterease}
            />
            <FileDropButton
              mutation={useIntuitMutation}
              title="Intuit"
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
              className="fade-in"
            />
          )}
          {submitMut.isError && (
            <CircleAlert color={red500} className="fade-in" />
          )}
          {submitMut.isError && <CircleAlert color={red500} />}
        </CardFooter>
      </Card>
    </div>
  );
}
