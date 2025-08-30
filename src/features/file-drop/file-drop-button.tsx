import { UseMutationResult } from "@tanstack/react-query";
import { useDropZone } from "./dropzone.ts";
import { Button } from "@/components/ui/button.tsx";
import { FileCheck, Loader, TriangleAlert, Upload } from "lucide-react";
import { Dispatch, SetStateAction, useEffect } from "react";
import { green500 } from "./constants.ts";

type FileDropProps = {
  mutation: () => UseMutationResult<unknown, Error, string, unknown>;
  setter: Dispatch<SetStateAction<boolean>>;
  title: string;
};

export function FileDropButton(props: FileDropProps) {
  const { mutation: useOurMutation, setter, title } = props;

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
  } else if (ourMut.isSuccess) {
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
        className="w-36"
      >
        {icon} {title}
      </Button>
      {ourMut.isError && ourMut.error.message}
    </div>
  );
}
