import { UseMutationResult } from "@tanstack/react-query";
import { FileCheck, Loader, TriangleAlert, Upload } from "lucide-react";
import { green500 } from "./constants.ts";
import { useDropZone } from "./dropzone.ts";
import {
  Tooltip,
  TooltipContent,
  TooltipTrigger,
} from "@/components/ui/tooltip.tsx";
import { open } from "@tauri-apps/plugin-dialog";
import { desktopDir } from "@tauri-apps/api/path";

type FileDropProps = {
  mutation: UseMutationResult<string, Error, string, unknown>;
  reset: () => void;
  title: string;
  tooltipContent: string;
};

export function FileDropButton(props: FileDropProps) {
  const { mutation: ourMut, reset, title, tooltipContent } = props;

  const { ref } = useDropZone({ onDrop: handleDrop });

  async function handleClick(): Promise<void> {
    const file = await open({
      multiple: false,
      directory: false,
      defaultPath: await desktopDir(),
      filters: [{ name: "Excel", extensions: ["xlsx"] }],
    });

    if (file) {
      sendIt(file);
    }
  }

  function handleDrop(paths: string[]): void {
    const filePath = paths[0];

    sendIt(filePath);
  }

  function sendIt(filePath: string): void {
    if (!filePath.endsWith(".xlsx")) return;

    ourMut.mutate(filePath);
    reset();
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
    <Tooltip delayDuration={500}>
      <TooltipTrigger>
        <button
          type="button"
          ref={ref}
          onClick={handleClick}
          className="
      h-36 w-36 select-none bg-background
      relative flex flex-col justify-center items-center
      transition-all 
      border-4 border-dotted hover:border-yellow-400 rounded-lg      
      "
        >
          <div className="flex flex-col justify-center items-center text-lg">
            {icon} {title}
          </div>
          <span className="
            absolute bottom-2 left-2 right-2
            text-muted-foreground text-sm truncate
            ">
            {ourMut.data}
          </span>
        </button>
      </TooltipTrigger>
      <TooltipContent>
        <p>{tooltipContent}</p>
      </TooltipContent>
    </Tooltip>
  );
}
