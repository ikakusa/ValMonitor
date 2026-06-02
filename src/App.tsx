import { AppLayout } from "@/components/layout/app-layout";
import { useDiscordRpc } from "@/app/use-discord-rpc";
import { Skeleton } from "@/components/ui/skeleton";
import { useUiStore } from "@/stores/ui-store";
import { lazy, Suspense } from "react";

const AccountPage = lazy(() =>
  import("@/features/account/account-page").then((module) => ({ default: module.AccountPage })),
);
const LivePage = lazy(() =>
  import("@/features/live/live-page").then((module) => ({ default: module.LivePage })),
);
const SettingsPage = lazy(() =>
  import("@/features/settings/settings-page").then((module) => ({ default: module.SettingsPage })),
);

export default function App() {
  const page = useUiStore((state) => state.page);
  useDiscordRpc();

  return (
    <AppLayout>
      <Suspense fallback={<PageFallback />}>
        {page === "account" && <AccountPage />}
        {page === "live" && <LivePage />}
        {page === "settings" && <SettingsPage />}
      </Suspense>
    </AppLayout>
  );
}

function PageFallback() {
  return (
    <div className="space-y-4 p-4 md:p-6">
      <Skeleton className="h-40 w-full" />
      <div className="grid gap-4 md:grid-cols-3">
        <Skeleton className="h-32" />
        <Skeleton className="h-32" />
        <Skeleton className="h-32" />
      </div>
    </div>
  );
}
