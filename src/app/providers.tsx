import { QueryClient, QueryClientProvider } from "@tanstack/react-query";
import type { ReactNode } from "react";

import { TooltipProvider } from "@/components/ui/tooltip";

const queryClient = new QueryClient({
  defaultOptions: {
    queries: {
      retry: 1,
      staleTime: 800,
    },
  },
});

export function AppProviders({ children }: { children: ReactNode }) {
  return (
    <QueryClientProvider client={queryClient}>
      <TooltipProvider>{children}</TooltipProvider>
    </QueryClientProvider>
  );
}
