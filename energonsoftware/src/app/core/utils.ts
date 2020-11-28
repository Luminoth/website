import * as moment from 'moment';

const TimestampFormat = 'L LTS ZZ';

//#region Types

export interface IDictionary<T> { [key: string]: T; }

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
  return moment.unix(timestamp).format(TimestampFormat);
}

//#endregion
