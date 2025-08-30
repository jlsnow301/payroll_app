import { QueryClient, QueryClientProvider } from "@tanstack/react-query";
import { ThemeProvider } from "./components/theme-provider";
import { FileDropPage } from "./features/file-drop";

const queryClient = new QueryClient();

export function App() {
  return (
    <QueryClientProvider client={queryClient}>
      <ThemeProvider defaultTheme="dark" storageKey="vite-ui-theme">
        <main className="h-screen w-screen p-2 bg-background text-foreground flex">
          <FileDropPage />
        </main>
      </ThemeProvider>
    </QueryClientProvider>
  );
}
