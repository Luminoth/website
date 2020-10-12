import {
  Component, OnInit, AfterViewInit,
  ChangeDetectionStrategy, ChangeDetectorRef
} from '@angular/core';
import { Title, Meta } from '@angular/platform-browser';
import { MatSnackBar } from '@angular/material/snack-bar';

import { DownloadsService } from '../../services/downloads.service';

import { IDownloadCategory, IDownload } from '../../core/downloads';
import { IDictionary } from '../../core/utils';

enum State {
  Idle,
  Loading
}

@Component({
  selector: 'app-downloads',
  templateUrl: './downloads.component.html',
  styleUrls: ['./downloads.component.scss'],
  changeDetection: ChangeDetectionStrategy.OnPush
})
export class DownloadsComponent implements OnInit, AfterViewInit {
  readonly State = State;
  private _state = State.Idle;

  private _loadingDownloadCategories = false;
  private _loadingDownloads = false;

  downloadCategories: IDictionary<IDownloadCategory> = {};
  downloads: IDictionary<IDownload[]> = {};

  //#region Lifecycle

  constructor(private title: Title,
    private meta: Meta,
    private cd: ChangeDetectorRef,
    private snackBar: MatSnackBar,
    private downloadsService: DownloadsService) {
  }

  ngOnInit() {
    this.title.setTitle('Energon Software - Downloads');
    this.meta.updateTag({
      name: 'description',
      content: 'Downloads',
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

  private idle() {
    if (!this._loadingDownloadCategories
      && !this._loadingDownloads) {
      this.state = State.Idle;
    }
  }

  get hasDownloads() {
    return Object.keys(this.downloads).length > 0;
  }

  getDownloadCategory(downloadCategoryId: string) {
    return this.downloadCategories[downloadCategoryId] ?? {
      id: 'missing',
      title: 'MISSING',
      decription: 'DOWNLOAD CATEGORY MISSING',
    };
  }

  private async getDataAsync() {
    await Promise.all([
      this.getDownloadCategoriesAsync(),
      this.getDownloadsAsync(),
    ]);
  }

  private async getDownloadCategoriesAsync() {
    this.state = State.Loading;
    this._loadingDownloadCategories = true;
    try {
      const response = await this.downloadsService.getDownloadCategoriesAsync();

      this.downloadCategories = {};
      for (const downloadCategory of response.download_categories) {
        this.downloadCategories[downloadCategory.id] = downloadCategory;
      }
    } catch (error) {
      this.snackBar.open(`Download Categories Load Error: ${error}`, 'OK', {
        panelClass: 'warn',
      });
    } finally {
      this._loadingDownloadCategories = false;
      this.idle();
    }
  }

  private async getDownloadsAsync() {
    this.state = State.Loading;
    this._loadingDownloads = true;
    try {
      const response = await this.downloadsService.getDownloadsAsync();

      this.downloads = {};
      for (const download of response.downloads) {
        if (!Object.keys(this.downloads).includes(download.category)) {
          this.downloads[download.category] = [];
        }
        this.downloads[download.category].push(download);
      }
    } catch (error) {
      this.snackBar.open(`Downloads Load Error: ${error}`, 'OK', {
        panelClass: 'warn',
      });
    } finally {
      this._loadingDownloads = false;
      this.idle();
    }
  }

}
