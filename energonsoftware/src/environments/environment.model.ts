export interface NewRelicBrowserConfig {
  accountID: number;
  trustKey: number;
  agentID: number;
  licenseKey: string;
  applicationID: number;
}

export interface Environment {
  production: boolean;
  apiHost: string;
  staticUrl: string;
  newRelic: NewRelicBrowserConfig | null;
}
