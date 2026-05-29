import { ComponentFixture, TestBed } from '@angular/core/testing';
import { provideRouter } from '@angular/router';
import { provideHttpClient } from '@angular/common/http';
import { provideHttpClientTesting } from '@angular/common/http/testing';

import { WoWScreenshotsRagnarosComponent } from './wow-screenshots-ragnaros.component';

describe('WoWScreenshotsRagnarosComponent', () => {
  let component: WoWScreenshotsRagnarosComponent;
  let fixture: ComponentFixture<WoWScreenshotsRagnarosComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      imports: [WoWScreenshotsRagnarosComponent],
      providers: [provideRouter([]), provideHttpClient(), provideHttpClientTesting()],
    }).compileComponents();

    fixture = TestBed.createComponent(WoWScreenshotsRagnarosComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
