import { async, ComponentFixture, TestBed } from '@angular/core/testing';

import { WowMacrosComponent } from './wow-macros.component';

describe('WowMacrosComponent', () => {
  let component: WowMacrosComponent;
  let fixture: ComponentFixture<WowMacrosComponent>;

  beforeEach(async(() => {
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
