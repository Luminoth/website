import { ComponentFixture, TestBed, waitForAsync } from '@angular/core/testing';

import { WowMacrosComponent } from './wow-macros.component';

describe('WowMacrosComponent', () => {
  let component: WowMacrosComponent;
  let fixture: ComponentFixture<WowMacrosComponent>;

  beforeEach(waitForAsync(() => {
    TestBed.configureTestingModule({
      declarations: [ WowMacrosComponent ]
    })
    .compileComponents();
  }));

  beforeEach(() => {
    fixture = TestBed.createComponent(WowMacrosComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
