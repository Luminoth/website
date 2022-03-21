import { Injectable } from '@angular/core';
import { HttpClient } from '@angular/common/http';
import { lastValueFrom } from 'rxjs';

import { environment } from '../../environments/environment';

import {
  IGetNewsAuthorsResponse, IGetNewsResponse
} from '../core/news';

@Injectable({
  providedIn: 'root'
})
export class NewsService {

  //#region Lifecycle

  constructor(private http: HttpClient) {
  }

  //#endregion

  //#region Commands

  getNewsAuthors() {
    return this.http.get<IGetNewsAuthorsResponse>(
      `${environment.apiHost}/v1/news/authors`
    );
  }

  async getNewsAuthorsAsync() {
    return lastValueFrom(this.getNewsAuthors());
  }

  getNews() {
    return this.http.get<IGetNewsResponse>(
      `${environment.apiHost}/v1/news`
    );
  }

  async getNewsAsync() {
    return lastValueFrom(this.getNews());
  }

  //#endregion

}
