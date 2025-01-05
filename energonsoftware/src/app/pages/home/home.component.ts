import {
  Component, OnInit,
  ChangeDetectionStrategy
} from '@angular/core';
import { Title, Meta } from '@angular/platform-browser';

@Component({
  selector: 'app-home',
  templateUrl: './home.component.html',
  styleUrls: ['./home.component.scss'],
  changeDetection: ChangeDetectionStrategy.OnPush,
  standalone: false
})
export class HomeComponent implements OnInit {

  //#region Lifecycle

  constructor(private title: Title,
    private meta: Meta) {
  }

  ngOnInit() {
    this.title.setTitle('Energon Software - Home');
    this.meta.updateTag({
      name: 'description',
      content: 'Downloads',
    });
  }

  //#endregion

}
