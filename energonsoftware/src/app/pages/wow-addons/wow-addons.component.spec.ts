import { async, ComponentFixture, TestBed } from '@angular/core/testing';

import { WowAddonsComponent } from './wow-addons.component';

describe('WowAddonsComponent', () => {
  let component: WowAddonsComponent;
  let fixture: ComponentFixture<WowAddonsComponent>;

  beforeEach(async(() => {
    TestBed.configureTestingModule({
      declarations: [ WowAddonsComponent ]
    })
    .compileComponents();
  }));

  beforeEach(() => {
    fixture = TestBed.createComponent(WowAddonsComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
