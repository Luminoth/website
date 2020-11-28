//#region Types

export interface IPictures {
  images: string[];
  text: string;
}

//#endregion

//#region Messages

export interface IGetPicturesResponse {
  pictures: IPictures[];
}

//#endregion
