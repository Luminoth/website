import { ComponentFixture, TestBed, waitForAsync } from '@angular/core/testing';

import { KennelComponent } from './kennel.component';

describe('KennelComponent', () => {
  let component: KennelComponent;
  let fixture: ComponentFixture<KennelComponent>;

  beforeEach(waitForAsync(() => {
    TestBed.configureTestingModule({
      declarations: [ KennelComponent ]
    })
    .compileComponents();
  }));

  beforeEach(() => {
    fixture = TestBed.createComponent(KennelComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
