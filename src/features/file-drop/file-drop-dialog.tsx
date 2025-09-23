import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogHeader,
  DialogTitle,
  DialogTrigger,
} from "@/components/ui/dialog.tsx";
import { CircleQuestionMark } from "lucide-react";

type Props = {
  description: string;
  headers: string[] | undefined;
  title: string;
};

export function FileDropDialog(props: Props) {
  const { description, headers = ["Unknown"], title } = props;

  return (
    <Dialog>
      <DialogTrigger
        asChild
        className="text-muted-foreground hover:text-foreground transition-colors"
      >
        <CircleQuestionMark />
      </DialogTrigger>
      <DialogContent>
        <DialogHeader>
          <DialogTitle>{title}</DialogTitle>
          <DialogDescription>{description}</DialogDescription>
        </DialogHeader>
        <span className="text-sm text-muted-foreground">
          Expected headers:
        </span>
        <ol className="list-decimal list-inside">
          {headers.map((header) => (
            <li className="text-sm" key={header}>{header}</li>
          ))}
        </ol>
      </DialogContent>
    </Dialog>
  );
}
