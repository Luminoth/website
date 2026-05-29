import { IAddon, IMacro, IScreenshots } from './wow';

describe('wow', () => {
  it('IAddon is usable', () => {
    const a: IAddon = { name: 'n', version: '1.0', latest_version: true, enabled: true, url: 'u', description: 'd' };
    expect(a.name).toBe('n');
  });

  it('IMacro is usable', () => {
    const m: IMacro = { name: 'n', macro: '/cast Fireball' };
    expect(m.macro).toContain('Fireball');
  });

  it('IScreenshots is usable', () => {
    const s: IScreenshots = { images: ['a.jpg'], text: 'caption' };
    expect(s.images.length).toBe(1);
  });
});
