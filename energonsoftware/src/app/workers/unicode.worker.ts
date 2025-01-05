/// <reference lib="webworker" />
import lodash from 'lodash';

import { IUnicode } from '../core/unicode';

addEventListener('message', ({ data }) => {
  const unicode: IUnicode[][] = [];

  let row = -1;
  for (const i of lodash.range(2 ** 16)) {
    if (i % 16 === 0) {
      row++;
      unicode[row] = [];
    }

    unicode[row].push({
      value: i,
      hexValue: i.toString(16).toUpperCase(),
      character: `&#${i};`,
    });
  }

  postMessage(unicode);
});
