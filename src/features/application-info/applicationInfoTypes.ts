export type ApplicationEnvironment = "development" | "production";

export type ApplicationStatus = "ready";

export type DatabaseStatus = "not_configured";

export interface DatabaseInfo {
  status: DatabaseStatus;
  version: string | null;
}

export interface ApplicationInfo {
  name: string;
  version: string;
  environment: ApplicationEnvironment;
  status: ApplicationStatus;
  database: DatabaseInfo;
}