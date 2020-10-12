import { Injectable } from '@angular/core';
import { Router } from '@angular/router';
import { HttpInterceptor, HttpRequest, HttpEvent, HttpHandler, HttpErrorResponse } from '@angular/common/http';
import { Observable, throwError } from 'rxjs';
import { retry, catchError } from 'rxjs/operators';

@Injectable()
export class HttpErrorInterceptor implements HttpInterceptor {

  //#region Lifecycle

  constructor(private router: Router) {
  }

  //#endregion

  intercept(req: HttpRequest<any>, next: HttpHandler): Observable<HttpEvent<any>> {
    return next.handle(req)
      .pipe(
        retry(1),
        catchError((error: HttpErrorResponse, _: Observable<any>) => {
          console.error(error);

          const errorMessage = error.error && error.error.message
            ? error.error.message
            : error.statusText;
          return throwError(errorMessage);
        })
      );
  }

}
