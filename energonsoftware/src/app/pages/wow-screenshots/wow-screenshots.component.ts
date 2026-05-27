import {
  Component, OnInit, AfterViewInit,
  ChangeDetectionStrategy, ChangeDetectorRef
} from '@angular/core';
import { Title, Meta } from '@angular/platform-browser';
import { MatSnackBar } from '@angular/material/snack-bar';
import { inject } from '@angular/core';
import { RouterModule } from '@angular/router';
import { NgbCarouselModule } from '@ng-bootstrap/ng-bootstrap';
import { MatProgressBarModule } from '@angular/material/progress-bar';

import { environment } from '../../../environments/environment';

import { WoWService } from '../../services/wow.service';

import { IScreenshots } from '../../core/wow';

enum State {
  Idle,
  Loading
}

@Component({
  selector: 'app-wow-screenshots',
  templateUrl: './wow-screenshots.component.html',
  styleUrls: ['./wow-screenshots.component.scss'],
  changeDetection: ChangeDetectionStrategy.OnPush,
  standalone: true,
  imports: [RouterModule, MatProgressBarModule, NgbCarouselModule],
})
export class WoWScreenshotsComponent implements OnInit, AfterViewInit {
  readonly State = State;
  private _state = State.Idle;

  readonly staticUrl = `${environment.staticUrl}/images/wow/screenshots`;
  screenshots: IScreenshots[] = [];

  private title = inject(Title);
  private meta = inject(Meta);
  private cd = inject(ChangeDetectorRef);
  private snackBar = inject(MatSnackBar);
  private wowService = inject(WoWService);

  ngOnInit() {
    this.title.setTitle('Energon Software - World of Warcraft Screenshots');
    this.meta.updateTag({
      name: 'description',
      content: 'World of Warcraft Screenshots',
    });
  }

  ngAfterViewInit() {
    this.getDataAsync();
  }

  get state() {
    return this._state;
  }

  set state(state: State) {
    this._state = state;

    this.cd.detectChanges();
  }

  get hasScreenshots() {
    return this.screenshots.length > 0;
  }

  getImageUrl(imageId: string) {
    return `${this.staticUrl}/${imageId}.jpg`;
  }

  private async getDataAsync() {
    await this.getScreenshotsAsync();
  }

  private async getScreenshotsAsync() {
    this.screenshots = [];

    this.state = State.Loading;
    try {
      const response = await this.wowService.getScreenshotsAsync();

      // fix a design mistake
      for (const screenshot of response.screenshots) {
        for (const image of screenshot.images) {
          this.screenshots.push({
            images: [image],
            text: screenshot.text,
          });
        }
      }
    } catch (error) {
      this.snackBar.open(`WoW Screenshots Load Error: ${error}`, 'OK', {
        panelClass: 'es-warn',
      });
    } finally {
      this.state = State.Idle;
    }
  }
}
