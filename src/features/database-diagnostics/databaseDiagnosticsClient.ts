import { invokeCommand } from "../../lib/tauriClient";
import type { DatabaseDiagnostics } from "./databaseDiagnosticsTypes";

export async function getDatabaseDiagnostics(): Promise<DatabaseDiagnostics> {
  return invokeCommand<DatabaseDiagnostics>("get_database_diagnostics");
}