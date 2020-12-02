import { ComponentFixture, TestBed, waitForAsync } from '@angular/core/testing';

import { UnicodeComponent } from './unicode.component';

describe('UnicodeComponent', () => {
  let component: UnicodeComponent;
  let fixture: ComponentFixture<UnicodeComponent>;

  beforeEach(waitForAsync(() => {
    TestBed.configureTestingModule({
      declarations: [ UnicodeComponent ]
    })
    .compileComponents();
  }));

  beforeEach(() => {
    fixture = TestBed.createComponent(UnicodeComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
