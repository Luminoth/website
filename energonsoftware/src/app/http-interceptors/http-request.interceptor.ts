
import { Injectable } from '@angular/core';
import { HttpInterceptor, HttpHandler, HttpRequest } from '@angular/common/http';

import { environment } from '../../environments/environment';

@Injectable()
export class HttpRequestInterceptor implements HttpInterceptor {

  intercept(req: HttpRequest<unknown>, next: HttpHandler) {
    if (!environment.production) {
      console.log(req);
    }

    return next.handle(req);
  }

}
