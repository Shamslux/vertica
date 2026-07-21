import { invoke } from "@tauri-apps/api/core";

export async function invokeCommand<TResponse>(
  command: string,
): Promise<TResponse> {
  return invoke<TResponse>(command);
}