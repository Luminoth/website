import { Environment } from './environment.model';

export const environment: Environment = {
  production: true,
  apiHost: 'https://api.energonsoftware.org',
  staticUrl: 'https://energonsoftware-static.s3.amazonaws.com',
  newRelic: {
    accountID: 7615751,
    trustKey: 7615751,
    agentID: 653435450,
    licenseKey: 'NRJS-b1caaa6e03cfd41eb93',
    applicationID: 653435450,
  },
};
