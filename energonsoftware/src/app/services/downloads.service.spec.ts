import { TestBed } from '@angular/core/testing';
import { provideHttpClient } from '@angular/common/http';
import { provideHttpClientTesting } from '@angular/common/http/testing';

import { DownloadsService } from './downloads.service';

describe('DownloadsService', () => {
  let service: DownloadsService;

  beforeEach(() => {
    TestBed.configureTestingModule({
      providers: [provideHttpClient(), provideHttpClientTesting()],
    });
    service = TestBed.inject(DownloadsService);
  });

  it('should be created', () => {
    expect(service).toBeTruthy();
  });
});
