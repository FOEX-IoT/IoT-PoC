import { Injectable } from '@angular/core';
import { BehaviorSubject } from 'rxjs';
import { Lamp, Status } from '../models/lamp';

@Injectable({
  providedIn: 'root',
})
export class LampsService {
  lamps: BehaviorSubject<Lamp[]> = new BehaviorSubject<Lamp[]>([
    {
      brightness: 12,
      id: 123,
      name: 'test111',
      status: 'on',
    },
    {
      brightness: 33,
      id: 12111,
      name: 't22222',
      status: 'on',
    },
    {
      brightness: 1,
      id: 1256,
      name: '31233333',
      status: 'off',
    },
  ]);
  activeLamp: BehaviorSubject<number> = new BehaviorSubject<number>(123);
  constructor() {}

  changeBrightness(brightness: number, id: number) {
    let previous = [...this.lamps.getValue()];
    let target = previous.find((lamp) => lamp.id === id);
    target.brightness = brightness;
    console.log(brightness);
    this.lamps.next(previous);
  }

  toggleStatus(id: number) {
    let previous = [...this.lamps.getValue()];
    let target = previous.find((lamp) => lamp.id === id);
    if (target.status === 'off') {
      target.status = 'on';
    } else {
      target.status = 'off';
    }
    this.lamps.next(previous);
  }

  setStatus(id: number, status: Status) {
    let previous = [...this.lamps.getValue()];
    let target = previous.find((lamp) => lamp.id === id);
    target.status = status;
    this.lamps.next(previous);
  }
}
