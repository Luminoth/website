import { ComponentFixture, TestBed, waitForAsync } from '@angular/core/testing';

import { WowScreenshotsComponent } from './wow-screenshots.component';

describe('WowScreenshotsComponent', () => {
  let component: WowScreenshotsComponent;
  let fixture: ComponentFixture<WowScreenshotsComponent>;

  beforeEach(waitForAsync(() => {
    TestBed.configureTestingModule({
      declarations: [ WowScreenshotsComponent ]
    })
    .compileComponents();
  }));

  beforeEach(() => {
    fixture = TestBed.createComponent(WowScreenshotsComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
