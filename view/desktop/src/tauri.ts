import { invoke } from "@tauri-apps/api/core";
import type { InvokeArgs } from "@tauri-apps/api/core";

// prettier-ignore
type TauriCommand = 
    | "sidebar_get_all_activities";

export async function invokeCmd<T>(cmd: TauriCommand, args?: InvokeArgs): Promise<T> {
  return invoke(cmd, args);
}
