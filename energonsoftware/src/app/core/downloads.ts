//#region Types

export interface IDownloadCategory {
  id: string;

  title: string;
  description: string;
}

export interface IDownload {
  id: string;

  name: string;
  category: string;
  url: string;
  description: string;
  version?: string;
}

//#endregion

//#region Messages

export interface IGetDownloadCategoriesResponse {
  download_categories: IDownloadCategory[];
}

export interface IGetDownloadsResponse {
  downloads: IDownload[];
}

////#endregion
