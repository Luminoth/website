import {
  Component, OnInit,
  ChangeDetectionStrategy
} from '@angular/core';
import { Title, Meta } from '@angular/platform-browser';

import { environment } from '../../../environments/environment';

@Component({
  selector: 'app-socket',
  templateUrl: './socket.component.html',
  styleUrls: ['./socket.component.scss'],
  changeDetection: ChangeDetectionStrategy.OnPush,
  standalone: false
})
export class SocketComponent implements OnInit {
  readonly environment = environment;

  //#region Lifecycle

  constructor(private title: Title,
    private meta: Meta) {
    this.title.setTitle('Energon Software - Simple, Portable, Socket Tutorial in C');
    this.meta.updateTag({
      name: 'description',
      content: 'Socket Tutorial in C',
    });
  }

  ngOnInit() {
  }

  //#endregion

}
