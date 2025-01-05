import {
  Component, OnInit, AfterViewInit,
  ChangeDetectionStrategy, ChangeDetectorRef
} from '@angular/core';
import { Title, Meta } from '@angular/platform-browser';
import { MatSnackBar } from '@angular/material/snack-bar';

import { WoWService } from '../../services/wow.service';

import { IMacroClass } from '../../core/wow';
import { stringCompare } from '../../core/utils';

enum State {
  Idle,
  Loading
}

@Component({
    selector: 'app-wow-macros',
    templateUrl: './wow-macros.component.html',
    styleUrls: ['./wow-macros.component.scss'],
    changeDetection: ChangeDetectionStrategy.OnPush,
    standalone: false
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
    private wowService: WoWService) {
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

  get hasMacros() {
    return this.macroClasses.length > 0;
  }

  private async getDataAsync() {
    await this.getMacrosAsync();
  }

  private async getMacrosAsync() {
    this.macroClasses = [];

    this.state = State.Loading;
    try {
      const response = await this.wowService.getMacrosAsync();

      this.macroClasses = response.macro_classes.sort((x, y) => {
        // some special casing to push General to the top
        if (x.character_class === 'General') {
          return -1;
        }
        if (y.character_class === 'General') {
          return 1;
        }
        return stringCompare(x.character_class, y.character_class);
      });
    } catch (error) {
      this.snackBar.open(`WoW Macros Load Error: ${error}`, 'OK', {
        panelClass: 'es-warn',
      });
    } finally {
      this.state = State.Idle;
    }
  }

}
