import { Injectable } from "@angular/core";
import { Observable, Subject } from "rxjs/Rx";
import { WebsocketService } from "./websocket.service";

const CHAT_URL = "ws://localhost:8080/ws/";

@Injectable({
  providedIn: 'root'
})
export class MessageService {

  public messages: Subject<any>;

  constructor(private _wsService: WebsocketService) {
    this.messages = <Subject<any>>_wsService.connect(CHAT_URL).map(
      (response: MessageEvent): any => {
        let data = JSON.parse(response.data);
        return data;
      }
    );
  }
}
