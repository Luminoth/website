import { IPictures } from './pictures';

describe('pictures', () => {
  it('IPictures is usable', () => {
    const p: IPictures = { images: ['a.jpg'], text: 'caption' };
    expect(p.images.length).toBe(1);
  });
});
