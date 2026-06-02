import { callCommand } from "@/services/tauri/client";

const DISCORD_CLIENT_ID = "1485730113550028902";

export type DiscordRpcActivityInput = {
  details?: string;
  state?: string;
};

export const DiscordRPC = {
  setActivity: (input: DiscordRpcActivityInput) =>
    callCommand<void>("discord_rpc_set_activity", {
      input: {
        clientId: DISCORD_CLIENT_ID,
        ...input,
      },
    }),
  clear: () => callCommand<void>("discord_rpc_clear"),
};
