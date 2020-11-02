import {
  Component, OnInit,
  ChangeDetectionStrategy
} from '@angular/core';
import { Title, Meta } from '@angular/platform-browser';

@Component({
  selector: 'app-oct2005cabin',
  templateUrl: './oct2005cabin.component.html',
  styleUrls: ['./oct2005cabin.component.scss'],
  changeDetection: ChangeDetectionStrategy.OnPush
})
export class Oct2005CabinComponent implements OnInit {

  //#region Lifecycle

  constructor(private title: Title,
    private meta: Meta) {
  }

  ngOnInit() {
    this.title.setTitle('Energon Software - October 2005 Cabin Trip');
    this.meta.updateTag({
      name: 'description',
      content: 'October 2005 Cabin Trip',
    });
  }

  //#endregion

}
