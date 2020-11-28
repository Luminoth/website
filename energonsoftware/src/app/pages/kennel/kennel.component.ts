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

  staticUrl: string;

  //#region Lifecycle

  constructor(private title: Title,
    private meta: Meta) {
    this.staticUrl = `${environment.staticUrl}/images/kennel`;
  }

  ngOnInit() {
    this.title.setTitle('Energon Software - Kennel Pictures');
    this.meta.updateTag({
      name: 'description',
      content: 'Kennel Pics',
    });
  }

  //#endregion

  getImageUrl(imageId: string) {
    return `${this.staticUrl}/IMG_00${imageId}.JPG`;
  }

}
