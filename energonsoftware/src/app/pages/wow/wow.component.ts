import {
  Component, OnInit,
  ChangeDetectionStrategy
} from '@angular/core';
import { Title, Meta } from '@angular/platform-browser';
import { inject } from '@angular/core';
import { RouterModule } from '@angular/router';
import { MatExpansionModule } from '@angular/material/expansion';

@Component({
  selector: 'app-wow',
  templateUrl: './wow.component.html',
  styleUrls: ['./wow.component.scss'],
  changeDetection: ChangeDetectionStrategy.OnPush,
  standalone: true,
  imports: [RouterModule, MatExpansionModule],
})
export class WoWComponent implements OnInit {

  private title = inject(Title);
  private meta = inject(Meta);

  ngOnInit() {
    this.title.setTitle('Energon Software - World of Warcraft');
    this.meta.updateTag({
      name: 'description',
      content: 'World of Warcraft',
    });
  }

}
