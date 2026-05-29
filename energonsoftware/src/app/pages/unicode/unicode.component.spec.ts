import { ComponentFixture, TestBed } from '@angular/core/testing';

import { IUnicode } from '../../core/unicode';
import { UnicodeComponent } from './unicode.component';

const makeRow = (start: number): IUnicode[] =>
  Array.from({ length: 16 }, (_, i) => ({
    value: start + i,
    hexValue: (start + i).toString(16).toUpperCase().padStart(4, '0'),
    character: String.fromCodePoint(start + i),
  }));

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

  describe('getCellHex', () => {
    it('returns hexValue at the column index (base 16)', () => {
      const row = makeRow(0x41); // A=0041, B=0042, ...
      expect(component.getCellHex(row, '0')).toBe('0041');
      expect(component.getCellHex(row, '1')).toBe('0042');
      expect(component.getCellHex(row, 'F')).toBe('0050');
    });
  });

  describe('getCharacter', () => {
    it('returns character at the column index (base 16)', () => {
      const row = makeRow(0x41);
      expect(component.getCharacter(row, '0')).toBe('A');
      expect(component.getCharacter(row, '1')).toBe('B');
      expect(component.getCharacter(row, 'F')).toBe('P');
    });
  });
});
