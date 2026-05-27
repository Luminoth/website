import {
  Component, OnInit,
  ChangeDetectionStrategy
} from '@angular/core';
import { Title, Meta } from '@angular/platform-browser';
import { inject } from '@angular/core';

@Component({
  selector: 'app-home',
  templateUrl: './home.component.html',
  styleUrls: ['./home.component.scss'],
  changeDetection: ChangeDetectionStrategy.OnPush,
  standalone: true,
})
export class HomeComponent implements OnInit {

  private title = inject(Title);
  private meta = inject(Meta);

  ngOnInit() {
    this.title.setTitle('Energon Software - Home');
    this.meta.updateTag({
      name: 'description',
      content: 'Downloads',
    });
  }

}
