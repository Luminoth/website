import { TestBed } from '@angular/core/testing';

import { SafeHtmlPipe } from './safe-html.pipe';

describe('SafeHtmlPipe', () => {
  beforeEach(() => {
    TestBed.configureTestingModule({
      imports: [SafeHtmlPipe],
    });
  });

  it('create an instance', () => {
    const pipe = TestBed.runInInjectionContext(() => new SafeHtmlPipe());
    expect(pipe).toBeTruthy();
  });
});
