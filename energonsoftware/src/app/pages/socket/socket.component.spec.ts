import { ComponentFixture, TestBed, waitForAsync } from '@angular/core/testing';

import { SocketComponent } from './socket.component';

describe('SocketComponent', () => {
  let component: SocketComponent;
  let fixture: ComponentFixture<SocketComponent>;

  beforeEach(waitForAsync(() => {
    TestBed.configureTestingModule({
      declarations: [ SocketComponent ]
    })
    .compileComponents();
  }));

  beforeEach(() => {
    fixture = TestBed.createComponent(SocketComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
