import { ComponentFixture, TestBed, waitForAsync } from '@angular/core/testing';

import { WowAddonsComponent } from './wow-addons.component';

describe('WowAddonsComponent', () => {
  let component: WowAddonsComponent;
  let fixture: ComponentFixture<WowAddonsComponent>;

  beforeEach(waitForAsync(() => {
    TestBed.configureTestingModule({
      declarations: [ WowAddonsComponent ]
    })
    .compileComponents();
  }));

  beforeEach(() => {
    fixture = TestBed.createComponent(WowAddonsComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
