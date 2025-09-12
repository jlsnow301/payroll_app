import { QueryClient, QueryClientProvider } from "@tanstack/react-query";
import { ThemeProvider } from "./components/theme-provider";
import { HomePage } from "./pages/home";
import { Page, useSimpleRouter } from "./hooks";
import { ReviewPage } from "./pages/review";

const queryClient = new QueryClient();

export function App() {
  const [page] = useSimpleRouter();

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
