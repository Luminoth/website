import {
  Component, OnInit,
  ChangeDetectionStrategy
} from '@angular/core';
import { Title, Meta } from '@angular/platform-browser';
import { inject } from '@angular/core';
import { MatDividerModule } from '@angular/material/divider';

@Component({
  selector: 'app-raw-xp',
  templateUrl: './raw-xp.component.html',
  styleUrls: ['./raw-xp.component.scss'],
  changeDetection: ChangeDetectionStrategy.OnPush,
  standalone: true,
  imports: [MatDividerModule],
})
export class RawXPComponent implements OnInit {

  private title = inject(Title);
  private meta = inject(Meta);

  ngOnInit() {
    this.title.setTitle('Energon Software - Thoughts on Raw Sockets in WindowsXP');
    this.meta.updateTag({
      name: 'description',
      content: 'Raw Sockets in WindowsXP Tutorial',
    });
  }

}
