import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import { Skeleton } from "@/components/ui/skeleton";
import { useTranslation } from "react-i18next";

export function StatusCard({
  title,
  value,
  detail,
  loading,
}: {
  title: string;
  value?: string | number | null;
  detail?: string;
  loading?: boolean;
}) {
  const { t } = useTranslation();

  return (
    <Card>
      <CardHeader className="pb-3">
        <CardTitle className="text-muted-foreground">{title}</CardTitle>
      </CardHeader>
      <CardContent>
        {loading ? (
          <Skeleton className="h-8 w-28" />
        ) : (
          <>
            <p className="truncate text-2xl font-semibold">{value ?? t("common.unknown")}</p>
            {detail && <p className="mt-1 truncate text-xs text-muted-foreground">{detail}</p>}
          </>
        )}
      </CardContent>
    </Card>
  );
}
