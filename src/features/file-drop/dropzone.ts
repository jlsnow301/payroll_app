import { listen } from "@tauri-apps/api/event";
import { useEffect, useRef } from "react";

type Point = {
  x: number;
  y: number;
};

type TauriDragDropEvent = {
  paths: string[];
  position: Point;
};

type Props = {
  onDrop: (paths: string[]) => void;
};

export function useDropZone(props: Props) {
  const { onDrop } = props;
  const ref = useRef<HTMLButtonElement>(null);

  useEffect(() => {
    const unlisten = listen<TauriDragDropEvent>(
      "tauri://drag-drop",
      (evt) => {
        const { x, y } = evt.payload.position;
        if (document.elementFromPoint(x, y) == ref.current) {
          onDrop(evt.payload.paths);
        }
      },
    );
    return () => {
      unlisten.then((unlisten) => {
        unlisten();
      });
    };
  }, [onDrop]);

  return { ref };
}
