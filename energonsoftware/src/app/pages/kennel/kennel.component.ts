import {
  Component, OnInit,
  ChangeDetectionStrategy
} from '@angular/core';
import { Title, Meta } from '@angular/platform-browser';
import { inject } from '@angular/core';
import { NgbCarouselModule } from '@ng-bootstrap/ng-bootstrap';
import lodash from 'lodash';

import { environment } from '../../../environments/environment';

@Component({
  selector: 'app-kennel',
  templateUrl: './kennel.component.html',
  styleUrls: ['./kennel.component.scss'],
  changeDetection: ChangeDetectionStrategy.OnPush,
  standalone: true,
  imports: [NgbCarouselModule],
})
export class KennelComponent implements OnInit {
  readonly lodash = lodash;
  readonly staticUrl = `${environment.staticUrl}/images/kennel`;

  private title = inject(Title);
  private meta = inject(Meta);

  ngOnInit() {
    this.title.setTitle('Energon Software - Kennel Pictures');
    this.meta.updateTag({
      name: 'description',
      content: 'Kennel Pics',
    });
  }

  getImageUrl(imageId: number) {
    return `${this.staticUrl}/IMG_00${imageId}.JPG`;
  }

}
