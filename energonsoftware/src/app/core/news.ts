//#region Types

export interface INewsAuthor {
  id: string;

  username: string;
  email_address: string;
  first_name: string;
  last_name: string;
}

export interface INews {
  id: string;

  title: string;
  timestamp: number;
  summary: string;
  author: string;
  news: string;
}

//#endregion

//#region Messages

export interface IGetNewsAuthorsResponse {
  news_authors: INewsAuthor[];
}

export interface IGetNewsResponse {
  news: INews[];
}

//#endregion
