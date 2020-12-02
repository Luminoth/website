import { ComponentFixture, TestBed, waitForAsync } from '@angular/core/testing';

import { WowComponent } from './wow.component';

describe('WowComponent', () => {
  let component: WowComponent;
  let fixture: ComponentFixture<WowComponent>;

  beforeEach(waitForAsync(() => {
    TestBed.configureTestingModule({
      declarations: [ WowComponent ]
    })
    .compileComponents();
  }));

  beforeEach(() => {
    fixture = TestBed.createComponent(WowComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
