import { QueryClient, QueryClientProvider } from "@tanstack/react-query";
import { relaunch } from "@tauri-apps/plugin-process";
import { check } from "@tauri-apps/plugin-updater";
import { useEffect } from "react";
import { ThemeProvider } from "./components/theme-provider.tsx";
import { Page, useSimpleRouter } from "./hooks.ts";
import { HomePage } from "./pages/home/index.tsx";
import { ReviewPage } from "./pages/review/index.tsx";

const queryClient = new QueryClient();

export function App() {
  const [page] = useSimpleRouter();

  useEffect(() => {
    checkUpdates();
  }, []);

  return (
    <QueryClientProvider client={queryClient}>
      <ThemeProvider defaultTheme="dark" storageKey="vite-ui-theme">
        <main className="h-screen w-screen text-foreground flex">
          {page === Page.Home && <HomePage />}
          {page === Page.Review && <ReviewPage />}
        </main>
      </ThemeProvider>
    </QueryClientProvider>
  );
}

enum UpdateEvent {
  Started = "Started",
  Progress = "Progress",
  Finished = "Finished",
}

/**
 * Straight from the docs
 * @see https://v2.tauri.app/plugin/updater/#checking-for-updates
 */
async function checkUpdates() {
  console.log("Checking for updates");

  const update = await check();
  if (!update) {
    console.log("No update available");
    return;
  }

  console.log(
    `found update ${update.version} from ${update.date} with notes ${update.body}`,
  );
  let downloaded = 0;
  let contentLength = 0;

  await update.downloadAndInstall((event) => {
    switch (event.event) {
      case UpdateEvent.Started:
        contentLength = event.data.contentLength || 0;
        console.log(
          `started downloading ${event.data.contentLength} bytes`,
        );
        break;
      case UpdateEvent.Progress:
        downloaded += event.data.chunkLength;
        console.log(`downloaded ${downloaded} from ${contentLength}`);
        break;
      case UpdateEvent.Finished:
        console.log("download finished");
        break;
    }
  });

  console.log("update installed");
  await relaunch();
}
