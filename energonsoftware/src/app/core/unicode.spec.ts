import { IUnicode } from './unicode';

describe('unicode', () => {
  it('IUnicode is usable', () => {
    const u: IUnicode = { value: 65, hexValue: '0041', character: 'A' };
    expect(u.character).toBe('A');
  });
});
