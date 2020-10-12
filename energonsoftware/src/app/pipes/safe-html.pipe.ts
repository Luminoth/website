import { Pipe, PipeTransform } from '@angular/core';
import { DomSanitizer } from '@angular/platform-browser';

@Pipe({
  name: 'safeHtml'
})
export class SafeHtmlPipe implements PipeTransform {

  //#region Lifecycle

  constructor(private sanitized: DomSanitizer) {
  }

  //#endregion

  transform(value: string) {
    return this.sanitized.bypassSecurityTrustHtml(value);
  }

}
