import { Injectable } from '@angular/core';
import { BehaviorSubject } from 'rxjs';
import { Lamp, Status } from '../models/lamp';
import { MessageService } from '../message.service'
import { HttpService } from '../http.service';
import { TYPES } from '../models/ws';
import { ScenesService } from '../shared/scenes.service'

@Injectable({
  providedIn: 'root',
})
export class LampsService {
  lamps: BehaviorSubject<Lamp[]> = new BehaviorSubject<Lamp[]>([]);
  // map mit number als key, und BehaviorSubject<Lamp> als value
  // im lamps component haben wir dann eine extra lamp component die immer alleine auf das bestimmte behaviorsubject listened
  
  activeLamp: BehaviorSubject<number> = new BehaviorSubject<number>(2005);

  constructor(private _msgService: MessageService, private _httpService: HttpService, private _scenesService: ScenesService) {
    _msgService.wsSubject.subscribe(dataFromServer => {
      if (dataFromServer.type === TYPES.lamp) {
        let previous = [...this.lamps.getValue()];
        let i = -1;
        let target = previous.find((lamp, index) => {
          i = index;
          return lamp.instanceId === dataFromServer.values.instanceId
        });
        let newTarget = {...target, ...dataFromServer.values};
        previous[i] = newTarget;
        this.lamps.next(previous);
      }
    })
    this.fetchLamps();
  }

  fetchLamps() {
    this._httpService.getLamps().subscribe(newLamps => {
      this.lamps.next(newLamps);
    })
  }

  changeBrightness(brightness: number, id: number) {
    let previous = [...this.lamps.getValue()];``
    let target = previous.find((lamp) => lamp.instanceId === id);
    target.brightness = brightness;
    this.lamps.next(previous);
    this._httpService.changeBrightnessOfLamp(id, brightness).subscribe(() => {});
    this._scenesService.setScene("CUSTOM");
  }

  toggleStatus(id: number) {
    let previous = [...this.lamps.getValue()];
    let target = previous.find((lamp) => lamp.instanceId === id);
    this._httpService.changeStatusOfLamp(id, !target.status).subscribe(() => {});
    this._scenesService.setScene("CUSTOM");
  }

  setStatus(id: number, status: boolean) {
    let previous = [...this.lamps.getValue()];
    let target = previous.find((lamp) => lamp.instanceId === id);
    target.status = status;
    this.lamps.next(previous);
    this._httpService.changeStatusOfLamp(id, status).subscribe(() => {});
    this._scenesService.setScene("CUSTOM")
  }
}
