import { QueryClient, QueryClientProvider } from "@tanstack/react-query";
import { ThemeProvider } from "./components/theme-provider";
import { HomePage } from "./pages/home";

const queryClient = new QueryClient();

export function App() {
  return (
    <QueryClientProvider client={queryClient}>
      <ThemeProvider defaultTheme="dark" storageKey="vite-ui-theme">
        <main className="h-screen w-screen text-foreground flex">
          <HomePage />
        </main>
      </ThemeProvider>
    </QueryClientProvider>
  );
}
