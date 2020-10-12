//#region Types

export interface IAddon {
  name: string;
  version: string;
  latest_version: string;
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

//#endregion

//#region Messages

export interface IGetAddonsResponse {
  wow_version: string;
  addons: IAddon[];
}

export interface IGetMacrosResponse {
  macros: IMacroClass[];
}

export interface IGetScreenshotsResponse {
  images: string[];
}

//#endregion
