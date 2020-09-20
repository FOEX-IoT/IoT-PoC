import { Injectable } from '@angular/core';
import { TYPES } from "../models/ws";
import { BehaviorSubject } from 'rxjs';
import { Scenes } from '../models/scene';
import { HttpService } from '../http.service';
import { MessageService } from '../message.service'

@Injectable({
  providedIn: 'root',
})
export class ScenesService {
  activeScene = new BehaviorSubject<Scenes>('CUSTOM');

  constructor(private httpService: HttpService, private _msgService: MessageService) {
    _msgService.wsSubject.subscribe(dataFromServer => {
      if (dataFromServer.type === TYPES.scenes) {
        this.activeScene.next(dataFromServer.values.scene);
      }
    })
  }

  setScene(scene: Scenes) {
    this.httpService.changeScene(scene).subscribe(() => {});
  }
}
