import {
  Component, OnInit,
  ChangeDetectionStrategy
} from '@angular/core';
import { Title, Meta } from '@angular/platform-browser';
import * as lodash from 'lodash';

import { environment } from '../../../environments/environment';

@Component({
  selector: 'app-kennel',
  templateUrl: './kennel.component.html',
  styleUrls: ['./kennel.component.scss'],
  changeDetection: ChangeDetectionStrategy.OnPush
})
export class KennelComponent implements OnInit {
  readonly lodash = lodash;
  readonly staticUrl = environment.staticUrl;

  //#region Lifecycle

  constructor(private title: Title,
    private meta: Meta) {
  }

  ngOnInit() {
    this.title.setTitle('Energon Software - Kennel Pictures');
    this.meta.updateTag({
      name: 'description',
      content: 'Kennel Pics',
    });
  }

  //#endregion

}
