import {
  Component, OnInit, AfterViewInit,
  ChangeDetectionStrategy, ChangeDetectorRef
} from '@angular/core';
import { Title, Meta } from '@angular/platform-browser';
import { MatSnackBar } from '@angular/material/snack-bar';
import { MatTableDataSource } from '@angular/material/table';

import { WoWService } from '../../services/wow.service';

import { IAddon } from '../../core/wow';

enum State {
  Idle,
  Loading
}

@Component({
  selector: 'app-wow-addons',
  templateUrl: './wow-addons.component.html',
  styleUrls: ['./wow-addons.component.scss'],
  changeDetection: ChangeDetectionStrategy.OnPush
})
export class WoWAddonsComponent implements OnInit, AfterViewInit {
  readonly State = State;
  private _state = State.Idle;

  //#region Table

  displayedColumns = ['enabled', 'name', 'version', 'description'];
  dataSource = new MatTableDataSource<IAddon>();

  //#endregion

  wowVersion = '0.0';

  //#region Lifecycle

  constructor(private title: Title,
    private meta: Meta,
    private cd: ChangeDetectorRef,
    private snackBar: MatSnackBar,
    private wowService: WoWService) {
  }

  ngOnInit() {
    this.title.setTitle('Energon Software - World of Warcraft Addons');
    this.meta.updateTag({
      name: 'description',
      content: 'World of Warcraft Addons',
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

  get hasAddons() {
    return Object.keys(this.dataSource.data).length > 0;
  }

  private async getDataAsync() {
    await this.getAddonsAsync();
  }

  private async getAddonsAsync() {
    this.wowVersion = '0.0';
    this.dataSource.data = [];

    this.state = State.Loading;
    try {
      const response = await this.wowService.getAddonsAsync();

      this.wowVersion = response.wow_version;
      this.dataSource.data = response.addons;
    } catch (error) {
      this.snackBar.open(`WoW Addons Load Error: ${error}`, 'OK', {
        panelClass: 'es-warn',
      });
    } finally {
      this.state = State.Idle;
    }
  }

}
