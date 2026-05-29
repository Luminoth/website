import { TestBed } from '@angular/core/testing';
import { provideHttpClient } from '@angular/common/http';
import { HttpTestingController, provideHttpClientTesting } from '@angular/common/http/testing';

import { environment } from '../../environments/environment';
import { WoWService } from './wow.service';

describe('WoWService', () => {
  let service: WoWService;
  let httpTesting: HttpTestingController;

  beforeEach(() => {
    TestBed.configureTestingModule({
      providers: [provideHttpClient(), provideHttpClientTesting()],
    });
    service = TestBed.inject(WoWService);
    httpTesting = TestBed.inject(HttpTestingController);
  });

  afterEach(() => httpTesting.verify());

  it('should be created', () => {
    expect(service).toBeTruthy();
  });

  it('getAddons() makes GET to /v1/wow/addons', () => {
    service.getAddons().subscribe();
    const req = httpTesting.expectOne(`${environment.apiHost}/v1/wow/addons`);
    expect(req.request.method).toBe('GET');
    req.flush({ wow_version: '1.0', addons: [] });
  });

  it('getMacros() makes GET to /v1/wow/macros', () => {
    service.getMacros().subscribe();
    const req = httpTesting.expectOne(`${environment.apiHost}/v1/wow/macros`);
    expect(req.request.method).toBe('GET');
    req.flush({ macro_classes: [] });
  });

  it('getScreenshots() without id makes GET to /v1/wow/screenshots', () => {
    service.getScreenshots().subscribe();
    const req = httpTesting.expectOne(`${environment.apiHost}/v1/wow/screenshots`);
    expect(req.request.method).toBe('GET');
    req.flush({ screenshots: [] });
  });

  it('getScreenshots(id) makes GET to /v1/wow/screenshots/:id', () => {
    service.getScreenshots('ragnaros').subscribe();
    const req = httpTesting.expectOne(`${environment.apiHost}/v1/wow/screenshots/ragnaros`);
    expect(req.request.method).toBe('GET');
    req.flush({ screenshots: [] });
  });
});
