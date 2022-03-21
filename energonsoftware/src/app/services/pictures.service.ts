import { Injectable } from '@angular/core';
import { HttpClient } from '@angular/common/http';
import { lastValueFrom } from 'rxjs';

import { environment } from '../../environments/environment';

import {
  IGetPicturesResponse
} from '../core/pictures';

@Injectable({
  providedIn: 'root'
})
export class PicturesService {

  //#region Lifecycle

  constructor(private http: HttpClient) {
  }

  //#endregion

  //#region Commands

  getPictures(id: string) {
    return this.http.get<IGetPicturesResponse>(
      `${environment.apiHost}/v1/pictures/${id}`
    );
  }

  async getPicturesAsync(id: string) {
    return lastValueFrom(this.getPictures(id));
  }

  getVacationPictures(id: string) {
    return this.http.get<IGetPicturesResponse>(
      `${environment.apiHost}/v1/pictures/vacation/${id}`
    );
  }

  async getVacationPicturesAsync(id: string) {
    return lastValueFrom(this.getVacationPictures(id));
  }

  //#endregion

}
