
//#region Types

export type IDictionary<T> = Record<string, T>;

export function dictionarySize<T>(dict: IDictionary<T>) {
  return Object.keys(dict).length;
}

//#endregion

//#region Utils

export function stringCompare(x: string, y: string) {
  if (x > y) {
    return 1;
  }
  if (x < y) {
    return -1;
  }
  return 0;
}

export function formatTimestamp(timestamp: number) {
  return new Date(timestamp * 1000).toLocaleString(undefined, {
    dateStyle: 'short',
    timeStyle: 'long',
  });
}

//#endregion
