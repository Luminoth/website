import { TestBed } from '@angular/core/testing';
import { provideHttpClient } from '@angular/common/http';
import { provideHttpClientTesting } from '@angular/common/http/testing';

import { WoWService } from './wow.service';

describe('WoWService', () => {
  let service: WoWService;

  beforeEach(() => {
    TestBed.configureTestingModule({
      providers: [provideHttpClient(), provideHttpClientTesting()],
    });
    service = TestBed.inject(WoWService);
  });

  it('should be created', () => {
    expect(service).toBeTruthy();
  });
});
