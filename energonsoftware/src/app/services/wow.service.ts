import { Injectable } from '@angular/core';
import { HttpClient } from '@angular/common/http';
import { lastValueFrom } from 'rxjs';

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
    return lastValueFrom(this.getAddons());
  }

  getMacros() {
    return this.http.get<IGetMacrosResponse>(
      `${environment.apiHost}/v1/wow/macros`
    );
  }

  async getMacrosAsync() {
    return lastValueFrom(this.getMacros());
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
    return lastValueFrom(this.getScreenshots(id));
  }

  //#endregion

}
