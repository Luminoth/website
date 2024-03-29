import { Injectable } from '@angular/core';
import { HttpClient } from '@angular/common/http';
import { lastValueFrom } from 'rxjs';

import { environment } from '../../environments/environment';

import {
  IGetDownloadCategoriesResponse, IGetDownloadsResponse
} from '../core/downloads';

@Injectable({
  providedIn: 'root'
})
export class DownloadsService {

  //#region Lifecycle

  constructor(private http: HttpClient) {
  }

  //#endregion

  //#region Commands

  getDownloadCategories() {
    return this.http.get<IGetDownloadCategoriesResponse>(
      `${environment.apiHost}/v1/downloads/categories`
    );
  }

  async getDownloadCategoriesAsync() {
    return lastValueFrom(this.getDownloadCategories());
  }

  getDownloads() {
    return this.http.get<IGetDownloadsResponse>(
      `${environment.apiHost}/v1/downloads`
    );
  }

  async getDownloadsAsync() {
    return lastValueFrom(this.getDownloads());
  }

  //#endregion

}
