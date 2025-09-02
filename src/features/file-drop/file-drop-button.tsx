import { UseMutationResult } from "@tanstack/react-query";
import { FileCheck, Loader, TriangleAlert, Upload } from "lucide-react";
import { green500 } from "./constants.ts";
import { useDropZone } from "./dropzone.ts";
import {
  Tooltip,
  TooltipContent,
  TooltipTrigger,
} from "@/components/ui/tooltip.tsx";

type FileDropProps = {
  mutation: UseMutationResult<unknown, Error, string, unknown>;
  title: string;
  tooltipContent: string;
};

export function FileDropButton(props: FileDropProps) {
  const { mutation: ourMut, title, tooltipContent } = props;

  const { ref } = useDropZone({ onDrop: handleDrop });

  function handleDrop(paths: string[]): void {
    const filePath = paths[0];
    if (!filePath.endsWith(".xlsx")) return;

    ourMut.mutate(filePath);
  }

  let icon = <Upload size={45} />;
  if (ourMut.isError) {
    icon = <TriangleAlert size={45} />;
  } else if (ourMut.isPending) {
    icon = <Loader className="animate-spin" size={45} />;
  } else if (ourMut.isSuccess) {
    icon = <FileCheck color={green500} size={45} />;
  }

  return (
    <Tooltip>
      <TooltipTrigger>
        <div
          ref={ref}
          className="
      h-36 w-36 select-none bg-background gap-4
      flex flex-col justify-center items-center
      text-foreground text-lg
      transition-all
      border-4 border-dotted hover:border-yellow-400 rounded-lg      
      "
        >
          {icon} {title}
        </div>
      </TooltipTrigger>
      <TooltipContent>
        <p>{tooltipContent}</p>
      </TooltipContent>
    </Tooltip>
  );
}
