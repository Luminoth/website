import { ComponentFixture, TestBed } from '@angular/core/testing';
import { provideRouter } from '@angular/router';
import { provideHttpClient } from '@angular/common/http';
import { provideHttpClientTesting } from '@angular/common/http/testing';

import { WoWScreenshotsComponent } from './wow-screenshots.component';

describe('WoWScreenshotsComponent', () => {
  let component: WoWScreenshotsComponent;
  let fixture: ComponentFixture<WoWScreenshotsComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      imports: [WoWScreenshotsComponent],
      providers: [provideRouter([]), provideHttpClient(), provideHttpClientTesting()],
    }).compileComponents();

    fixture = TestBed.createComponent(WoWScreenshotsComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
