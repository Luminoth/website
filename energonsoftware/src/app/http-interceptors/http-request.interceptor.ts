
import { Injectable } from '@angular/core';
import { HttpInterceptor, HttpHandler, HttpRequest } from '@angular/common/http';

import { environment } from '../../environments/environment';

@Injectable()
export class HttpRequestInterceptor implements HttpInterceptor {

  //#region Lifecycle

  constructor() {
  }

  //#endregion

  intercept(req: HttpRequest<any>, next: HttpHandler) {
    if (!environment.production) {
      console.log(req);
    }

    return next.handle(req);
  }

}
