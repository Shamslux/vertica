import { invokeCommand } from "../../lib/tauriClient";
import type { ApplicationInfo } from "./applicationInfoTypes";

const GET_APPLICATION_INFO_COMMAND = "get_application_info";

export async function getApplicationInfo(): Promise<ApplicationInfo> {
  return invokeCommand<ApplicationInfo>(
    GET_APPLICATION_INFO_COMMAND,
  );
}