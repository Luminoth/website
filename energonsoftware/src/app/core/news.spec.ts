import { INews, INewsAuthor } from './news';

describe('news', () => {
  it('INews is usable', () => {
    const n: INews = { id: '1', title: 't', timestamp: 0, summary: 's', author: 'a', news: 'n' };
    expect(n.id).toBe('1');
  });

  it('INewsAuthor is usable', () => {
    const a: INewsAuthor = { id: '1', username: 'u', email_address: 'e', first_name: 'f', last_name: 'l' };
    expect(a.id).toBe('1');
  });
});
