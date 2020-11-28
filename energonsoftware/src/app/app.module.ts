import { BrowserModule } from '@angular/platform-browser';
import { BrowserAnimationsModule } from '@angular/platform-browser/animations';
import { NgModule } from '@angular/core';
import { FlexLayoutModule } from '@angular/flex-layout';
import { HttpClientModule } from '@angular/common/http';
import { NgbModule } from '@ng-bootstrap/ng-bootstrap';

import { httpInterceptorProviders } from './http-interceptors';

import { AppRoutingModule } from './app-routing.module';
import { AppMaterialModule } from './app-material.module';

import { AppComponent } from './app.component';
import { NavigationComponent } from './navigation/navigation.component';

// pages
import { NewsComponent } from './pages/news/news.component';
import { AboutComponent } from './pages/about/about.component';
import { DownloadsComponent } from './pages/downloads/downloads.component';
import { LinksComponent } from './pages/links/links.component';
import { HomeComponent } from './pages/home/home.component';
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

// pipes
import { SafeHtmlPipe } from './pipes/safe-html.pipe';

@NgModule({
  declarations: [
    AppComponent,
    NavigationComponent,

    // pages
    NewsComponent,
    AboutComponent,
    DownloadsComponent,
    LinksComponent,
    HomeComponent,
    RawXPComponent,
    NotepadComponent,
    SocketComponent,
    Oct2005CabinComponent,
    KennelComponent,
    UnicodeComponent,
    WoWComponent,
    WoWAddonsComponent,
    WoWMacrosComponent,
    WoWScreenshotsComponent,

    // pipes
    SafeHtmlPipe,
  ],
  imports: [
    BrowserModule,
    BrowserAnimationsModule,
    FlexLayoutModule,
    NgbModule,
    HttpClientModule,
    AppRoutingModule,
    AppMaterialModule,
  ],
  providers: [
    httpInterceptorProviders,
  ],
  bootstrap: [AppComponent]
})
export class AppModule { }
