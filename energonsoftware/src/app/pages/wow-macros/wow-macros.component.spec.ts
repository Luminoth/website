import { ComponentFixture, TestBed } from '@angular/core/testing';
import { provideHttpClient } from '@angular/common/http';
import { provideHttpClientTesting } from '@angular/common/http/testing';

import { WoWMacrosComponent } from './wow-macros.component';

describe('WoWMacrosComponent', () => {
  let component: WoWMacrosComponent;
  let fixture: ComponentFixture<WoWMacrosComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      imports: [WoWMacrosComponent],
      providers: [provideHttpClient(), provideHttpClientTesting()],
    }).compileComponents();

    fixture = TestBed.createComponent(WoWMacrosComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
