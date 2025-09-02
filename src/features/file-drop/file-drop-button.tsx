import { Button } from "@/components/ui/button.tsx";
import { UseMutationResult } from "@tanstack/react-query";
import { FileCheck, Loader, TriangleAlert, Upload } from "lucide-react";
import { Dispatch, SetStateAction, useEffect } from "react";
import { green500 } from "./constants.ts";
import { useDropZone } from "./dropzone.ts";

type FileDropProps = {
  mutation: () => UseMutationResult<unknown, Error, string, unknown>;
  setter: Dispatch<SetStateAction<boolean>>;
  state: boolean;
  title: string;
};

export function FileDropButton(props: FileDropProps) {
  const { mutation: useOurMutation, setter, state: isUploaded, title } = props;

  const { ref } = useDropZone({ onDrop: handleDrop });

  const ourMut = useOurMutation(); // forgive me sir tan

  function handleDrop(paths: string[]): void {
    const filePath = paths[0];
    if (!filePath.endsWith(".xlsx")) return;

    ourMut.mutate(filePath);
  }

  let icon = <Upload />;
  if (ourMut.isError) {
    icon = <TriangleAlert />;
  } else if (ourMut.isPending) {
    icon = <Loader className="animate-spin" />;
  } else if (isUploaded) {
    icon = <FileCheck color={green500} />;
  }

  useEffect(() => {
    if (ourMut.isSuccess) {
      setter(true);
    }
  }, [ourMut.isSuccess]);

  return (
    <div className="flex flex-col gap-2">
      <Button
        ref={ref}
        variant="outline"
        size="lg"
        className="h-36 w-36 text-lg flex-col select-none border-4 border-dotted hover:dark:border-yellow-400"
      >
        {icon} {title}
      </Button>
      {ourMut.isError && ourMut.error.message}
    </div>
  );
}
