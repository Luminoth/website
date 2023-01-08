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
import { Oct2005CabinComponent } from './pages/oct2005cabin/oct2005cabin.component';
import { UnicodeComponent } from './pages/unicode/unicode.component';
import { WoWComponent } from './pages/wow/wow.component';
import { WoWAddonsComponent } from './pages/wow-addons/wow-addons.component';
import { WoWMacrosComponent } from './pages/wow-macros/wow-macros.component';
import { WoWScreenshotsComponent } from './pages/wow-screenshots/wow-screenshots.component';
import { WoWScreenshotsRagnarosComponent } from './pages/wow-screenshots-ragnaros/wow-screenshots-ragnaros.component';

// TODO: this is super dumb, most of this is static content
// so is there a way we can just use a single generic component
// but switch out the title and the template?

// TODO: the wow ragnaros screenshots setup is really dumb,
// what we actually want is a more generic screenshot page
// that can load the general screenshots or a "type" of screenshots

const routes: Routes = [
  { path: 'news', component: NewsComponent },
  { path: 'about', component: AboutComponent },
  { path: 'downloads', component: DownloadsComponent },
  { path: 'links', component: LinksComponent },
  { path: 'raw_xp', component: RawXPComponent },
  { path: 'tutorials/notepad', component: NotepadComponent },
  { path: 'tutorials/socket', component: SocketComponent },
  { path: 'kennel', component: KennelComponent },
  { path: 'vacation/oct2005cabin', component: Oct2005CabinComponent },
  { path: 'unicode', component: UnicodeComponent },
  { path: 'wow', component: WoWComponent },
  { path: 'wow/addons', component: WoWAddonsComponent },
  { path: 'wow/macros', component: WoWMacrosComponent },
  { path: 'wow/screenshots', component: WoWScreenshotsComponent },
  { path: 'wow/screenshots/ragnaros', component: WoWScreenshotsRagnarosComponent },
  // TODO: this needs to route to static CDN ...
  // when that's working, the download items in the CDN can change back to the website URL
  // (instead of the S3 url), but then I guess we also need a directory browser, ugh
  //{ path: 'static/*', ??? },
  { path: '', component: HomeComponent, pathMatch: 'full' },
  { path: '**', redirectTo: '/' },
];

@NgModule({
  imports: [RouterModule.forRoot(routes)],
  exports: [RouterModule]
})
export class AppRoutingModule { }
