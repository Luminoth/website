import { dictionarySize, stringCompare, formatTimestamp } from './utils';

describe('utils', () => {
  describe('dictionarySize', () => {
    it('returns 0 for empty dict', () => expect(dictionarySize({})).toBe(0));
    it('counts keys', () => expect(dictionarySize({ a: 1, b: 2 })).toBe(2));
  });

  describe('stringCompare', () => {
    it('returns 0 for equal strings', () => expect(stringCompare('a', 'a')).toBe(0));
    it('returns positive when first > second', () => expect(stringCompare('b', 'a')).toBe(1));
    it('returns negative when first < second', () => expect(stringCompare('a', 'b')).toBe(-1));
  });

  describe('formatTimestamp', () => {
    it('returns a non-empty string', () => {
      expect(formatTimestamp(0).length).toBeGreaterThan(0);
    });
  });
});
