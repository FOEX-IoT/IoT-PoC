import { Injectable } from '@angular/core';
import { BehaviorSubject } from 'rxjs';

@Injectable({
  providedIn: 'root',
})
export class TemperatureService {
  private currentTemp = new BehaviorSubject<number>(23);
  constructor() {}

  getCurrentTemperature() {
    return this.currentTemp;
  }

  updateTemperature(temperature: number) {
    this.currentTemp.next(temperature);
  }
}
