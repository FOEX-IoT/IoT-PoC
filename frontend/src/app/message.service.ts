import { Injectable } from "@angular/core";
import {webSocket, WebSocketSubject} from 'rxjs/webSocket';


const WS_URL = "ws://localhost:8080/ws/";

@Injectable({
  providedIn: 'root'
})
export class MessageService {
  wsSubject: any;
  
  constructor() {
    this.wsSubject = webSocket(WS_URL).asObservable();
  } 
}