import { HTTP_INTERCEPTORS } from '@angular/common/http';

import { HttpRequestInterceptor } from './http-request.interceptor';
import { HttpResponseInterceptor } from './http-response.interceptor';
import { HttpErrorInterceptor } from './http-error.interceptor';

export const httpInterceptorProviders = [
  { provide: HTTP_INTERCEPTORS, useClass: HttpRequestInterceptor, multi: true },
  { provide: HTTP_INTERCEPTORS, useClass: HttpResponseInterceptor, multi: true },
  { provide: HTTP_INTERCEPTORS, useClass: HttpErrorInterceptor, multi: true },
];
