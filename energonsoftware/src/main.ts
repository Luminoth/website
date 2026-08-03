import { provideZoneChangeDetection } from '@angular/core';
import { bootstrapApplication } from '@angular/platform-browser';
import { provideRouter } from '@angular/router';
import { provideHttpClient, withInterceptorsFromDi, withXhr } from '@angular/common/http';

import { AppComponent } from './app/app.component';
import { routes } from './app/app-routing.module';
import { httpInterceptorProviders } from './app/http-interceptors';
import { initNewRelicBrowserAgent } from './app/new-relic-browser';

initNewRelicBrowserAgent();

bootstrapApplication(AppComponent, {
  providers: [
    provideZoneChangeDetection(),
    provideRouter(routes),
    provideHttpClient(withXhr(), withInterceptorsFromDi()),
    httpInterceptorProviders,
  ]
}).catch(err => console.log(err));
