import { async, ComponentFixture, TestBed } from '@angular/core/testing';

import { Oct2005cabinComponent } from './oct2005cabin.component';

describe('Oct2005cabinComponent', () => {
  let component: Oct2005cabinComponent;
  let fixture: ComponentFixture<Oct2005cabinComponent>;

  beforeEach(async(() => {
    TestBed.configureTestingModule({
      declarations: [ Oct2005cabinComponent ]
    })
    .compileComponents();
  }));

  beforeEach(() => {
    fixture = TestBed.createComponent(Oct2005cabinComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
