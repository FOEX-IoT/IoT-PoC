import { Component } from '@angular/core';
import { HttpService } from './http.service';
import { Lamp } from './models/lamp';

@Component({
  selector: 'app-root',
  templateUrl: './app.component.html',
  styleUrls: ['./app.component.scss'],
})

export class AppComponent {
  title = 'frontend';

  onTouchEnd() {
    console.log('asdf');
  }
}