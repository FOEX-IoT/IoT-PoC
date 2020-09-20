import { Component } from '@angular/core';
import { MessageService } from './message.service'

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

  constructor(private chatService: MessageService) {
    chatService.messages.subscribe(msg => {
      console.log("Response from websocket: " + msg);
    });
  }
}