import { invoke } from "@tauri-apps/api/core";

import type {
  ApplicationSettings,
  UpdateApplicationSettingsInput,
} from "./settingsTypes";

export async function getSettings(): Promise<ApplicationSettings> {
  return invoke<ApplicationSettings>("get_settings");
}

export async function updateSettings(
  input: UpdateApplicationSettingsInput,
): Promise<ApplicationSettings> {
  return invoke<ApplicationSettings>("update_settings", {
    input,
  });
}