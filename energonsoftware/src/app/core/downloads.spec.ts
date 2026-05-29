import { IDownload, IDownloadCategory } from './downloads';

describe('downloads', () => {
  it('IDownload is usable', () => {
    const d: IDownload = { id: '1', name: 'n', category: 'c', url: 'u', description: 'd' };
    expect(d.id).toBe('1');
  });

  it('IDownloadCategory is usable', () => {
    const c: IDownloadCategory = { id: '1', title: 't', description: 'd' };
    expect(c.id).toBe('1');
  });
});
