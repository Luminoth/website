import {
  Component, OnInit,
  ChangeDetectionStrategy
} from '@angular/core';
import { Title, Meta } from '@angular/platform-browser';
import { inject } from '@angular/core';
import { MatDividerModule } from '@angular/material/divider';

import { environment } from '../../../environments/environment';

@Component({
  selector: 'app-socket',
  templateUrl: './socket.component.html',
  styleUrls: ['./socket.component.scss'],
  changeDetection: ChangeDetectionStrategy.OnPush,
  standalone: true,
  imports: [MatDividerModule],
})
export class SocketComponent implements OnInit {
  readonly environment = environment;

  private title = inject(Title);
  private meta = inject(Meta);

  ngOnInit() {
    this.title.setTitle('Energon Software - Simple, Portable, Socket Tutorial in C');
    this.meta.updateTag({
      name: 'description',
      content: 'Socket Tutorial in C',
    });
  }

}
