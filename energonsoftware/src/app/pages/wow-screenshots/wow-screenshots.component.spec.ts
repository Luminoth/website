import { async, ComponentFixture, TestBed } from '@angular/core/testing';

import { WowScreenshotsComponent } from './wow-screenshots.component';

describe('WowScreenshotsComponent', () => {
  let component: WowScreenshotsComponent;
  let fixture: ComponentFixture<WowScreenshotsComponent>;

  beforeEach(async(() => {
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
