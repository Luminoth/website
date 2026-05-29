import { ComponentFixture, TestBed } from '@angular/core/testing';
import { provideHttpClient } from '@angular/common/http';
import { provideHttpClientTesting } from '@angular/common/http/testing';

import { Oct2005CabinComponent } from './oct2005cabin.component';

describe('Oct2005CabinComponent', () => {
  let component: Oct2005CabinComponent;
  let fixture: ComponentFixture<Oct2005CabinComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      imports: [Oct2005CabinComponent],
      providers: [provideHttpClient(), provideHttpClientTesting()],
    }).compileComponents();

    fixture = TestBed.createComponent(Oct2005CabinComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
