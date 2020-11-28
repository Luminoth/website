import {
  Component, OnInit,
  ChangeDetectionStrategy
} from '@angular/core';
import { Title, Meta } from '@angular/platform-browser';

@Component({
  selector: 'app-wow-screenshots',
  templateUrl: './wow-screenshots.component.html',
  styleUrls: ['./wow-screenshots.component.scss'],
  changeDetection: ChangeDetectionStrategy.OnPush
})
export class WoWScreenshotsComponent implements OnInit {

  //#region Lifecycle

  constructor(private title: Title,
    private meta: Meta) {
  }

  ngOnInit() {
    this.title.setTitle('Energon Software - World of Warcraft Screenshots');
    this.meta.updateTag({
      name: 'description',
      content: 'World of Warcraft Screenshots',
    });
  }

  //#endregion

}
