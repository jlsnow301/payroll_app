import { Alert, AlertDescription, AlertTitle } from "@/components/ui/alert";
import { Button } from "@/components/ui/button";
import { AlertCircleIcon } from "lucide-react";

type Props = {
  errors: string[];
  reset: () => void;
};

export function ErrorAlert(props: Props) {
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
