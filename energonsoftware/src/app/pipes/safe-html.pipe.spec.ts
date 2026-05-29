import { TestBed } from '@angular/core/testing';
import { SafeValue } from '@angular/platform-browser';

import { SafeHtmlPipe } from './safe-html.pipe';

describe('SafeHtmlPipe', () => {
  let pipe: SafeHtmlPipe;

  beforeEach(() => {
    TestBed.configureTestingModule({
      imports: [SafeHtmlPipe],
    });
    pipe = TestBed.runInInjectionContext(() => new SafeHtmlPipe());
  });

  it('create an instance', () => {
    expect(pipe).toBeTruthy();
  });

  it('transform() returns a SafeValue', () => {
    const result = pipe.transform('<b>bold</b>');
    expect((result as SafeValue).toString()).toContain('SafeValue');
  });

  it('transform() preserves the original HTML in its string representation', () => {
    const result = pipe.transform('<b>bold</b>');
    expect(result.toString()).toContain('<b>bold</b>');
  });
});
