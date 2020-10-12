import {
  Component, OnInit,
  ChangeDetectionStrategy
} from '@angular/core';
import { Title, Meta } from '@angular/platform-browser';

@Component({
  selector: 'app-kennel',
  templateUrl: './kennel.component.html',
  styleUrls: ['./kennel.component.scss'],
  changeDetection: ChangeDetectionStrategy.OnPush
})
export class KennelComponent implements OnInit {

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
