import {
  Component, OnInit, AfterViewInit,
  ChangeDetectionStrategy, ChangeDetectorRef
} from '@angular/core';
import { Title, Meta } from '@angular/platform-browser';
import { MatSnackBar } from '@angular/material/snack-bar';
import { inject } from '@angular/core';
import { NgbCarouselModule } from '@ng-bootstrap/ng-bootstrap';
import { MatProgressBarModule } from '@angular/material/progress-bar';

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
  changeDetection: ChangeDetectionStrategy.OnPush,
  standalone: true,
  imports: [MatProgressBarModule, NgbCarouselModule],
})
export class Oct2005CabinComponent implements OnInit, AfterViewInit {
  readonly State = State;
  private _state = State.Idle;

  readonly staticUrl = `${environment.staticUrl}/images/vacation/oct2005cabin`;
  pictures: IPictures[] = [];

  private title = inject(Title);
  private meta = inject(Meta);
  private cd = inject(ChangeDetectorRef);
  private snackBar = inject(MatSnackBar);
  private picturesService = inject(PicturesService);

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
