import { TestBed } from '@angular/core/testing';
import { provideHttpClient } from '@angular/common/http';
import { HttpTestingController, provideHttpClientTesting } from '@angular/common/http/testing';

import { environment } from '../../environments/environment';
import { PicturesService } from './pictures.service';

describe('PicturesService', () => {
  let service: PicturesService;
  let httpTesting: HttpTestingController;

  beforeEach(() => {
    TestBed.configureTestingModule({
      providers: [provideHttpClient(), provideHttpClientTesting()],
    });
    service = TestBed.inject(PicturesService);
    httpTesting = TestBed.inject(HttpTestingController);
  });

  afterEach(() => httpTesting.verify());

  it('should be created', () => {
    expect(service).toBeTruthy();
  });

  it('getPictures(id) makes GET to /v1/pictures/:id', () => {
    service.getPictures('gallery1').subscribe();
    const req = httpTesting.expectOne(`${environment.apiHost}/v1/pictures/gallery1`);
    expect(req.request.method).toBe('GET');
    req.flush({ pictures: [] });
  });

  it('getVacationPictures(id) makes GET to /v1/pictures/vacation/:id', () => {
    service.getVacationPictures('oct2005cabin').subscribe();
    const req = httpTesting.expectOne(`${environment.apiHost}/v1/pictures/vacation/oct2005cabin`);
    expect(req.request.method).toBe('GET');
    req.flush({ pictures: [] });
  });
});
