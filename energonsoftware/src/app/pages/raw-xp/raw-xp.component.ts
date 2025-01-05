import {
  Component, OnInit,
  ChangeDetectionStrategy
} from '@angular/core';
import { Title, Meta } from '@angular/platform-browser';

@Component({
  selector: 'app-raw-xp',
  templateUrl: './raw-xp.component.html',
  styleUrls: ['./raw-xp.component.scss'],
  changeDetection: ChangeDetectionStrategy.OnPush,
  standalone: false
})
export class RawXPComponent implements OnInit {

  //#region Lifecycle

  constructor(private title: Title,
    private meta: Meta) {
    this.title.setTitle('Energon Software - Thoughts on Raw Sockets in WindowsXP');
    this.meta.updateTag({
      name: 'description',
      content: 'Raw Sockets in WindowsXP Tutorial',
    });
  }

  ngOnInit() {
  }

  //#endregion

}
