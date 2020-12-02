import { ComponentFixture, TestBed, waitForAsync } from '@angular/core/testing';

import { WowScreenshotsRagnarosComponent } from './wow-screenshots-ragnaros.component';

describe('WowScreenshotsRagnarosComponent', () => {
  let component: WowScreenshotsRagnarosComponent;
  let fixture: ComponentFixture<WowScreenshotsRagnarosComponent>;

  beforeEach(waitForAsync(() => {
    TestBed.configureTestingModule({
      declarations: [ WowScreenshotsRagnarosComponent ]
    })
    .compileComponents();
  }));

  beforeEach(() => {
    fixture = TestBed.createComponent(WowScreenshotsRagnarosComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
