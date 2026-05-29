import { TestBed } from '@angular/core/testing';
import { provideHttpClient } from '@angular/common/http';
import { provideHttpClientTesting } from '@angular/common/http/testing';

import { PicturesService } from './pictures.service';

describe('PicturesService', () => {
  let service: PicturesService;

  beforeEach(() => {
    TestBed.configureTestingModule({
      providers: [provideHttpClient(), provideHttpClientTesting()],
    });
    service = TestBed.inject(PicturesService);
  });

  it('should be created', () => {
    expect(service).toBeTruthy();
  });
});
