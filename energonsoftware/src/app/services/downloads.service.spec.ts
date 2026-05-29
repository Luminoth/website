import { TestBed } from '@angular/core/testing';
import { provideHttpClient } from '@angular/common/http';
import { HttpTestingController, provideHttpClientTesting } from '@angular/common/http/testing';

import { environment } from '../../environments/environment';
import { DownloadsService } from './downloads.service';

describe('DownloadsService', () => {
  let service: DownloadsService;
  let httpTesting: HttpTestingController;

  beforeEach(() => {
    TestBed.configureTestingModule({
      providers: [provideHttpClient(), provideHttpClientTesting()],
    });
    service = TestBed.inject(DownloadsService);
    httpTesting = TestBed.inject(HttpTestingController);
  });

  afterEach(() => httpTesting.verify());

  it('should be created', () => {
    expect(service).toBeTruthy();
  });

  it('getDownloadCategories() makes GET to /v1/downloads/categories', () => {
    service.getDownloadCategories().subscribe();
    const req = httpTesting.expectOne(`${environment.apiHost}/v1/downloads/categories`);
    expect(req.request.method).toBe('GET');
    req.flush({ download_categories: [] });
  });

  it('getDownloads() makes GET to /v1/downloads', () => {
    service.getDownloads().subscribe();
    const req = httpTesting.expectOne(`${environment.apiHost}/v1/downloads`);
    expect(req.request.method).toBe('GET');
    req.flush({ downloads: [] });
  });
});
