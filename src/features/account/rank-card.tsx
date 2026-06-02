import { motion } from "motion/react";
import { useTranslation } from "react-i18next";

import { Badge } from "@/components/ui/badge";
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import type { RankView } from "@/types/riot";
import unrankedIcon from "@/Assets/unranked.png";

export function RankCard({
  title,
  rank,
  showRr,
}: {
  title: string;
  rank: RankView;
  showRr?: boolean;
}) {
  const { t } = useTranslation();

  return (
    <motion.div initial={{ opacity: 0, y: 8 }} animate={{ opacity: 1, y: 0 }}>
      <Card className="overflow-hidden">
        <CardHeader className="flex-row items-center justify-between space-y-0 pb-3">
          <CardTitle>{title}</CardTitle>
          <Badge variant="muted">{rank.seasonName ?? t("account.season")}</Badge>
        </CardHeader>
        <CardContent className="flex items-center gap-4">
          <img
            alt=""
            className="h-16 w-16 shrink-0 object-contain"
            src={rank.icon ?? unrankedIcon}
          />
          <div className="min-w-0">
            <p className="truncate text-lg font-semibold">{rank.name}</p>
            <p className="font-mono text-xs text-muted-foreground">
              {t("common.tier")} {rank.tier}
              {showRr ? ` / ${rank.rr ?? 0} RR` : ""}
            </p>
          </div>
        </CardContent>
      </Card>
    </motion.div>
  );
}
