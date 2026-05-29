import { TestBed } from '@angular/core/testing';
import { provideHttpClient } from '@angular/common/http';
import { HttpTestingController, provideHttpClientTesting } from '@angular/common/http/testing';

import { environment } from '../../environments/environment';
import { NewsService } from './news.service';

describe('NewsService', () => {
  let service: NewsService;
  let httpTesting: HttpTestingController;

  beforeEach(() => {
    TestBed.configureTestingModule({
      providers: [provideHttpClient(), provideHttpClientTesting()],
    });
    service = TestBed.inject(NewsService);
    httpTesting = TestBed.inject(HttpTestingController);
  });

  afterEach(() => httpTesting.verify());

  it('should be created', () => {
    expect(service).toBeTruthy();
  });

  it('getNewsAuthors() makes GET to /v1/news/authors', () => {
    service.getNewsAuthors().subscribe();
    const req = httpTesting.expectOne(`${environment.apiHost}/v1/news/authors`);
    expect(req.request.method).toBe('GET');
    req.flush({ news_authors: [] });
  });

  it('getNews() makes GET to /v1/news', () => {
    service.getNews().subscribe();
    const req = httpTesting.expectOne(`${environment.apiHost}/v1/news`);
    expect(req.request.method).toBe('GET');
    req.flush({ news: [] });
  });
});
