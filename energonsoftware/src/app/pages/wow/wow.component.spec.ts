import { ComponentFixture, TestBed } from '@angular/core/testing';
import { provideRouter } from '@angular/router';

import { WoWComponent } from './wow.component';

describe('WoWComponent', () => {
  let component: WoWComponent;
  let fixture: ComponentFixture<WoWComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      imports: [WoWComponent],
      providers: [provideRouter([])],
    }).compileComponents();

    fixture = TestBed.createComponent(WoWComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
