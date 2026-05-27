import {
  Component, OnInit,
  ChangeDetectionStrategy
} from '@angular/core';
import { Title, Meta } from '@angular/platform-browser';
import { inject } from '@angular/core';
import { MatDividerModule } from '@angular/material/divider';

import { environment } from '../../../environments/environment';

@Component({
  selector: 'app-notepad',
  templateUrl: './notepad.component.html',
  styleUrls: ['./notepad.component.scss'],
  changeDetection: ChangeDetectionStrategy.OnPush,
  standalone: true,
  imports: [MatDividerModule],
})
export class NotepadComponent implements OnInit {
  readonly environment = environment;

  private title = inject(Title);
  private meta = inject(Meta);

  ngOnInit() {
    this.title.setTitle('Energon Software - Notepad Clone Tutorial in C');
    this.meta.updateTag({
      name: 'description',
      content: 'Notepad Tutorial',
    });
  }

}
