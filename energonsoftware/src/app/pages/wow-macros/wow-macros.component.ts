import {
  Component, OnInit, AfterViewInit,
  ChangeDetectionStrategy, ChangeDetectorRef
} from '@angular/core';
import { Title, Meta } from '@angular/platform-browser';
import { MatSnackBar } from '@angular/material/snack-bar';

import { WoWService } from '../../services/wow.service';

import { IMacroClass } from '../../core/wow';

enum State {
  Idle,
  Loading
}

@Component({
  selector: 'app-wow-macros',
  templateUrl: './wow-macros.component.html',
  styleUrls: ['./wow-macros.component.scss'],
  changeDetection: ChangeDetectionStrategy.OnPush
})
export class WoWMacrosComponent implements OnInit, AfterViewInit {
  readonly State = State;
  private _state = State.Idle;

  macroClasses: IMacroClass[] = [];

  //#region Lifecycle

  constructor(private title: Title,
    private meta: Meta,
    private cd: ChangeDetectorRef,
    private snackBar: MatSnackBar,
    private wow: WoWService) {
  }

  ngOnInit() {
    this.title.setTitle('Energon Software - World of Warcraft Macros');
    this.meta.updateTag({
      name: 'description',
      content: 'World of Warcraft Macros',
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

  private async getDataAsync() {
    await this.getMacrosAsync();
  }

  private async getMacrosAsync() {
    this.macroClasses = [];

    this.state = State.Loading;
    try {
      const response = await this.wow.getMacrosAsync();

      this.macroClasses = response.macro_classes;
    } catch (error) {
      this.snackBar.open(`WoW Macros Load Error: ${error}`, 'OK', {
        panelClass: 'warn',
      });
    } finally {
      this.state = State.Idle;
    }
  }

}
