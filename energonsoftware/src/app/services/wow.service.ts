import { Injectable } from '@angular/core';
import { HttpClient } from '@angular/common/http';

import { environment } from '../../environments/environment';

import {
  IGetAddonsResponse, IGetMacrosResponse, IGetScreenshotsResponse
} from '../core/wow';

@Injectable({
  providedIn: 'root'
})
export class WoWService {

  //#region Lifecycle

  constructor(private http: HttpClient) {
  }

  //#endregion

  //#region Commands

  getAddons() {
    return this.http.get<IGetAddonsResponse>(
      `${environment.apiHost}/v1/wow/addons`
    );
  }

  async getAddonsAsync() {
    return this.getAddons().toPromise();
  }

  getMacros() {
    return this.http.get<IGetMacrosResponse>(
      `${environment.apiHost}/v1/wow/macros`
    );
  }

  async getMacrosAsync() {
    return this.getMacros().toPromise();
  }

  getScreenshots(id?: string) {
    if (id) {
      return this.http.get<IGetScreenshotsResponse>(
        `${environment.apiHost}/v1/wow/screenshots/${id}`
      );
    } else {
      return this.http.get<IGetScreenshotsResponse>(
        `${environment.apiHost}/v1/wow/screenshots`
      );
    }
  }

  async getScreenshotsAsync(id?: string) {
    return this.getScreenshots(id).toPromise();
  }

  //#endregion

}
