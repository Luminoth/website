import {
  Component,
  ChangeDetectionStrategy
} from '@angular/core';

@Component({
  selector: 'app-navigation',
  templateUrl: './navigation.component.html',
  styleUrls: ['./navigation.component.scss'],
  changeDetection: ChangeDetectionStrategy.OnPush,
  standalone: false
})
export class NavigationComponent {

  readonly currentYear = new Date().getFullYear();

  //#region Lifecycle

  constructor() {
  }

  //#endregion

}
