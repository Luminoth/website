import { ComponentFixture, TestBed, waitForAsync } from '@angular/core/testing';

import { RawXPComponent } from './raw-xp.component';

describe('RawXPComponent', () => {
  let component: RawXPComponent;
  let fixture: ComponentFixture<RawXPComponent>;

  beforeEach(waitForAsync(() => {
    TestBed.configureTestingModule({
      declarations: [ RawXPComponent ]
    })
    .compileComponents();
  }));

  beforeEach(() => {
    fixture = TestBed.createComponent(RawXPComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
