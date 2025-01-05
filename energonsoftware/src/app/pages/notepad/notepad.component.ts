import {
  Component, OnInit,
  ChangeDetectionStrategy
} from '@angular/core';
import { Title, Meta } from '@angular/platform-browser';

import { environment } from '../../../environments/environment';

@Component({
  selector: 'app-notepad',
  templateUrl: './notepad.component.html',
  styleUrls: ['./notepad.component.scss'],
  changeDetection: ChangeDetectionStrategy.OnPush,
  standalone: false
})
export class NotepadComponent implements OnInit {
  readonly environment = environment;

  //#region Lifecycle

  constructor(private title: Title,
    private meta: Meta) {
    this.title.setTitle('Energon Software - Notepad Clone Tutorial in C');
    this.meta.updateTag({
      name: 'description',
      content: 'Notepad Tutorial',
    });
  }

  ngOnInit() {
  }

  //#endregion

}
