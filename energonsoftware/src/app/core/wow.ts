//#region Types

export interface IAddon {
  name: string;
  version: string;
  latest_version: boolean;
  enabled: boolean;
  url: string;
  description: string;
}

export interface IMacro {
  name: string;
  macro: string;
}

export interface IMacroClass {
  character_class: string;
  macros: IMacro[];
}

export interface IScreenshots {
  images: string[];
  text: string;
}

//#endregion

//#region Messages

export interface IGetAddonsResponse {
  wow_version: string;
  addons: IAddon[];
}

export interface IGetMacrosResponse {
  macro_classes: IMacroClass[];
}

export interface IGetScreenshotsResponse {
  images: IScreenshots[];
}

//#endregion
