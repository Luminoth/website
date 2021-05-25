import {
  Component, OnInit, AfterViewInit, ViewChild,
  ChangeDetectionStrategy, ChangeDetectorRef
} from '@angular/core';
import { Title, Meta } from '@angular/platform-browser';
import { MatPaginator } from '@angular/material/paginator';
import { MatTableDataSource } from '@angular/material/table';
import * as lodash from 'lodash';
import * as moment from 'moment';

import { IUnicode } from '../../core/unicode';

enum State {
  Idle,
  Loading
}

@Component({
  selector: 'app-unicode',
  templateUrl: './unicode.component.html',
  styleUrls: ['./unicode.component.scss'],
  changeDetection: ChangeDetectionStrategy.OnPush
})
export class UnicodeComponent implements OnInit, AfterViewInit {
  readonly State = State;
  private _state = State.Idle;

  //#region Table

  @ViewChild(MatPaginator) paginator!: MatPaginator;
  displayedColumns = lodash.range(16).map(x => x.toString(16).toUpperCase());
  dataSource = new MatTableDataSource<IUnicode[]>();

  // paging
  pageSizeOption = [64, 128, 256, 512, 1024];

  //#endregion

  //#region Lifecycle

  constructor(private title: Title,
    private meta: Meta,
    private cd: ChangeDetectorRef) {
  }

  ngOnInit() {
    this.title.setTitle('Energon Software - Unicode');
    this.meta.updateTag({
      name: 'description',
      content: 'Unicode',
    });
  }

  ngAfterViewInit() {
    this.dataSource.paginator = this.paginator;

    this.constructTable();
  }

  //#endregion

  get state() {
    return this._state;
  }

  set state(state: State) {
    this._state = state;

    this.cd.detectChanges();
  }

  getCellHex(row: IUnicode[], column: string) {
    const idx = parseInt(column, 16);
    return row[idx].hexValue;
  }

  getCharacter(row: IUnicode[], column: string) {
    const idx = parseInt(column, 16);
    return row[idx].character;
  }

  private constructTable() {
    if (typeof Worker === 'undefined') {
      console.error('Workers not supported!');
      return;
    }

    this.state = State.Loading;
    this.dataSource.data = [];

    const worker = new Worker(new URL('../../workers/unicode.worker', import.meta.url), { type: 'module' });

    const start = moment().valueOf();
    worker.onmessage = ({ data }) => {
      //console.log(`worker completed in ${moment().valueOf() - start}ms`);

      this.dataSource.data = data;

      this.state = State.Idle;
    };
    worker.postMessage('');
  }

}
