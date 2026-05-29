import { ComponentFixture, TestBed } from '@angular/core/testing';

import { UnicodeComponent } from './unicode.component';

describe('UnicodeComponent', () => {
  let component: UnicodeComponent;
  let fixture: ComponentFixture<UnicodeComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      imports: [UnicodeComponent],
    }).compileComponents();

    fixture = TestBed.createComponent(UnicodeComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
