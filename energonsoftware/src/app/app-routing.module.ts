import { NgModule } from '@angular/core';
import { Routes, RouterModule } from '@angular/router';

import { HomeComponent } from './pages/home/home.component';
import { NewsComponent } from './pages/news/news.component';
import { AboutComponent } from './pages/about/about.component';
import { DownloadsComponent } from './pages/downloads/downloads.component';
import { LinksComponent } from './pages/links/links.component';
import { RawXPComponent } from './pages/raw-xp/raw-xp.component';
import { NotepadComponent } from './pages/notepad/notepad.component';
import { SocketComponent } from './pages/socket/socket.component';
import { KennelComponent } from './pages/kennel/kennel.component';
import { UnicodeComponent } from './pages/unicode/unicode.component';

// TODO: this is super dumb, most of this is static content
// so is there a way we can just use a single generic component
// but switch out the title and the template?

const routes: Routes = [
  { path: 'news', component: NewsComponent },
  { path: 'about', component: AboutComponent },
  { path: 'downloads', component: DownloadsComponent },
  { path: 'links', component: LinksComponent },
  { path: 'raw_xp', component: RawXPComponent },
  { path: 'tutorials/notepad', component: NotepadComponent },
  { path: 'tutorials/socket', component: SocketComponent },
  { path: 'kennel', component: KennelComponent },
  { path: 'unicode', component: UnicodeComponent },
  { path: '', component: HomeComponent, pathMatch: 'full' },
  { path: '*', redirectTo: '/' },
];

@NgModule({
  imports: [RouterModule.forRoot(routes)],
  exports: [RouterModule]
})
export class AppRoutingModule { }
