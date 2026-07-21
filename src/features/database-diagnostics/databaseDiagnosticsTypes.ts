export interface DatabaseDiagnostics {
  reachable: boolean;
  databasePath: string;
  schemaVersion: number;
  pendingMigrations: number;
  foreignKeysEnabled: boolean;
  journalMode: string;
}
