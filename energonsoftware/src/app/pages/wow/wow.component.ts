import {
  Component, OnInit,
  ChangeDetectionStrategy
} from '@angular/core';
import { Title, Meta } from '@angular/platform-browser';

@Component({
    selector: 'app-wow',
    templateUrl: './wow.component.html',
    styleUrls: ['./wow.component.scss'],
    changeDetection: ChangeDetectionStrategy.OnPush,
    standalone: false
})
export class WoWComponent implements OnInit {

  //#region Lifecycle

  constructor(private title: Title,
    private meta: Meta) {
  }

  ngOnInit() {
    this.title.setTitle('Energon Software - World of Warcraft');
    this.meta.updateTag({
      name: 'description',
      content: 'World of Warcraft',
    });
  }

  //#endregion

}
