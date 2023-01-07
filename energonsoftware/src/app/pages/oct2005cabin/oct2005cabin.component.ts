import {
  Component, OnInit, AfterViewInit,
  ChangeDetectionStrategy, ChangeDetectorRef
} from '@angular/core';
import { Title, Meta } from '@angular/platform-browser';
import { MatLegacySnackBar as MatSnackBar } from '@angular/material/legacy-snack-bar';

import { environment } from '../../../environments/environment';

import { PicturesService } from '../../services/pictures.service';

import { IPictures } from '../../core/pictures';

enum State {
  Idle,
  Loading
}

@Component({
  selector: 'app-oct2005cabin',
  templateUrl: './oct2005cabin.component.html',
  styleUrls: ['./oct2005cabin.component.scss'],
  changeDetection: ChangeDetectionStrategy.OnPush
})
export class Oct2005CabinComponent implements OnInit, AfterViewInit {
  readonly State = State;
  private _state = State.Idle;

  staticUrl: string;
  pictures: IPictures[] = [];

  //#region Lifecycle

  constructor(private title: Title,
    private meta: Meta,
    private cd: ChangeDetectorRef,
    private snackBar: MatSnackBar,
    private picturesService: PicturesService) {
    this.staticUrl = `${environment.staticUrl}/images/vacation/oct2005cabin`;
  }

  ngOnInit() {
    this.title.setTitle('Energon Software - October 2005 Cabin Trip');
    this.meta.updateTag({
      name: 'description',
      content: 'October 2005 Cabin Trip',
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

  get hasPictures() {
    return this.pictures.length > 0;
  }

  // TODO: thumbnails can go away
  /*getImageThumbnailUrl(imageId: string) {
    return `${this.staticUrl}/thumbnails/${imageId}.jpg`;
  }*/

  getImageUrl(imageId: string) {
    return `${this.staticUrl}/${imageId}.jpg`;
  }

  private async getDataAsync() {
    await this.getPicturesAsync();
  }

  private async getPicturesAsync() {
    this.pictures = [];

    this.state = State.Loading;
    try {
      const response = await this.picturesService.getVacationPicturesAsync('oct2005cabin');

      // fix a design mistake
      for (const picture of response.pictures) {
        for (const image of picture.images) {
          this.pictures.push({
            images: [image],
            text: picture.text,
          });
        }
      }
    } catch (error) {
      this.snackBar.open(`Pictures Load Error: ${error}`, 'OK', {
        panelClass: 'es-warn',
      });
    } finally {
      this.state = State.Idle;
    }
  }

}
