
import { Injectable } from '@angular/core';
import { HttpInterceptor, HttpHandler, HttpRequest, HttpResponse } from '@angular/common/http';
import { tap } from 'rxjs/operators';

import { environment } from '../../environments/environment';

@Injectable()
export class HttpResponseInterceptor implements HttpInterceptor {

  //#region Lifecycle

  constructor() {
  }

  //#endregion

  intercept(req: HttpRequest<any>, next: HttpHandler) {
    return next.handle(req).pipe(
      tap(evt => {
        if (!(evt instanceof HttpResponse)) {
          return;
        }

        if (!environment.production) {
          console.log(evt);
        }
      })
    );
  }

}
