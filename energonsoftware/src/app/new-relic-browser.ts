import { BrowserAgent } from '@newrelic/browser-agent/loaders/browser-agent';

import { environment } from '../environments/environment';

/**
 * Starts New Relic's Browser RUM agent when `environment.newRelic` is
 * configured (production only — see environment.prod.ts). No-ops otherwise,
 * so local dev never reports to New Relic.
 */
export function initNewRelicBrowserAgent(): void {
  const config = environment.newRelic;
  if (!config?.licenseKey) {
    return;
  }

  new BrowserAgent({
    init: {
      distributed_tracing: { enabled: true },
      privacy: { cookies_enabled: true },
      ajax: { deny_list: ['bam.nr-data.net'] },
    },
    info: {
      beacon: 'bam.nr-data.net',
      errorBeacon: 'bam.nr-data.net',
      licenseKey: config.licenseKey,
      // Info.applicationID is typed as string; loader_config.applicationID
      // below stays numeric to match what New Relic's API actually returns.
      applicationID: String(config.applicationID),
      sa: 1,
    },
    loader_config: {
      accountID: config.accountID,
      trustKey: config.trustKey,
      agentID: config.agentID,
      licenseKey: config.licenseKey,
      applicationID: config.applicationID,
    },
  });
}
