import {
  Component, OnInit, AfterViewInit,
  ChangeDetectionStrategy, ChangeDetectorRef
} from '@angular/core';
import { Title, Meta } from '@angular/platform-browser';
import { MatSnackBar } from '@angular/material/snack-bar';

import { environment } from '../../../environments/environment';

import { WoWService } from '../../services/wow.service';

import { IScreenshots } from '../../core/wow';

enum State {
  Idle,
  Loading
}

@Component({
    selector: 'app-wow-screenshots-ragnaros',
    templateUrl: './wow-screenshots-ragnaros.component.html',
    styleUrls: ['./wow-screenshots-ragnaros.component.scss'],
    changeDetection: ChangeDetectionStrategy.OnPush,
    standalone: false
})
export class WoWScreenshotsRagnarosComponent implements OnInit, AfterViewInit {
  readonly State = State;
  private _state = State.Idle;

  staticUrl: string;
  screenshots: IScreenshots[] = [];

  //#region Lifecycle

  constructor(private title: Title,
    private meta: Meta,
    private cd: ChangeDetectorRef,
    private snackBar: MatSnackBar,
    private wowService: WoWService) {
    this.staticUrl = `${environment.staticUrl}/images/wow/screenshots/ragnaros`;
  }

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

  //#endregion

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

  // TODO: thumbnails can go away
  /*getImageThumbnailUrl(imageId: string) {
    return `${this.staticUrl}/${imageId}_small.jpg`;
  }*/

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
      const response = await this.wowService.getScreenshotsAsync('ragnaros');

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
