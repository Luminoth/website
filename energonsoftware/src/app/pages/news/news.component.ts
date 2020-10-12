import {
  Component, OnInit, AfterViewInit,
  ChangeDetectionStrategy, ChangeDetectorRef
} from '@angular/core';
import { Title, Meta } from '@angular/platform-browser';
import { MatSnackBar } from '@angular/material/snack-bar';

import { NewsService } from '../../services/news.service';

import { INewsAuthor, INews } from '../../core/news';
import { IDictionary, formatTimestamp } from '../../core/utils';

enum State {
  Idle,
  Loading
}

@Component({
  selector: 'app-news',
  templateUrl: './news.component.html',
  styleUrls: ['./news.component.scss'],
  changeDetection: ChangeDetectionStrategy.OnPush
})
export class NewsComponent implements OnInit, AfterViewInit {
  readonly formatTimestamp = formatTimestamp;

  readonly State = State;
  private _state = State.Idle;

  private _loadingNewsAuthors = false;
  private _loadingNews = false;

  newsAuthors: IDictionary<INewsAuthor> = {};
  news: INews[] = [];

  //#region Lifecycle

  constructor(private title: Title,
    private meta: Meta,
    private cd: ChangeDetectorRef,
    private snackBar: MatSnackBar,
    private newsService: NewsService) {
  }

  ngOnInit() {
    this.title.setTitle('Energon Software - News');
    this.meta.updateTag({
      name: 'description',
      content: 'News',
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
    if (!this._loadingNewsAuthors
      && !this._loadingNews) {
      this.state = State.Idle;
    }
  }

  get hasNews() {
    return this.news.length > 0;
  }

  getNewsAuthor(newsAuthorId: string) {
    return this.newsAuthors[newsAuthorId] ?? {
      id: 'missing',
      username: 'MISSING',
      email_address: 'MISSING',
      first_name: 'MISSING',
      last_name: 'MISSING',
    };
  }

  getNewsAuthorName(newsAuthorId: string) {
    const author = this.getNewsAuthor(newsAuthorId);
    return `${author.first_name} ${author.last_name}`;
  }

  private async getDataAsync() {
    await Promise.all([
      this.getNewsAuthorsAsync(),
      this.getNewsAsync(),
    ]);
  }

  private async getNewsAuthorsAsync() {
    this.state = State.Loading;
    this._loadingNewsAuthors = true;
    try {
      const response = await this.newsService.getNewsAuthorsAsync();

      this.newsAuthors = {};
      for (const newsAuthor of response.news_authors) {
        this.newsAuthors[newsAuthor.id] = newsAuthor;
      }
    } catch (error) {
      this.snackBar.open(`News Authors Load Error: ${error}`, 'OK', {
        panelClass: 'warn',
      });
    } finally {
      this._loadingNewsAuthors = false;
      this.idle();
    }
  }

  private async getNewsAsync() {
    this.state = State.Loading;
    this._loadingNews = true;
    try {
      const response = await this.newsService.getNewsAsync();

      this.news = response.news.sort((x, y) => y.timestamp - x.timestamp);
    } catch (error) {
      this.snackBar.open(`News Load Error: ${error}`, 'OK', {
        panelClass: 'warn',
      });
    } finally {
      this._loadingNews = false;
      this.idle();
    }
  }

}
