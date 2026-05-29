import { ComponentFixture, TestBed } from '@angular/core/testing';

import { RawXPComponent } from './raw-xp.component';

describe('RawXPComponent', () => {
  let component: RawXPComponent;
  let fixture: ComponentFixture<RawXPComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      imports: [RawXPComponent],
    }).compileComponents();

    fixture = TestBed.createComponent(RawXPComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
