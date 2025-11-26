import { AlertCircleIcon } from "lucide-react";
import {
  Alert,
  AlertDescription,
  AlertTitle,
} from "../../components/ui/alert.tsx";

type Props = {
  errors: string[];
};

export function ErrorAlert(props: Props) {
  const { errors } = props;

  return (
    <Alert
      variant="destructive"
      className="text-lg"
    >
      <AlertCircleIcon />
      <AlertTitle className="flex items-center justify-between">
        Error detected
      </AlertTitle>
      <AlertDescription className="max-h-20 overflow-y-auto">
        <ul className="list-disc list-inside px-2">
          {errors.map((msg, index) => <li key={index}>{msg}</li>)}
        </ul>
      </AlertDescription>
    </Alert>
  );
}
