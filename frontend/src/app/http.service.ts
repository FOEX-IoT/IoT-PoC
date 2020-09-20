import { Injectable } from '@angular/core';
import { HttpClient } from '@angular/common/http';
import { Lamp } from './models/lamp';
import { Scenes } from './models/scene';


const BASE_URL = "http://localhost:8080/api/";
const ALL_LAMPS = BASE_URL + "lamps/all";
const CHANGE_STATUS = BASE_URL + "lamps/update-status";
const CHANGE_BRIGHTNESS = BASE_URL + "lamps/update-brightness";
const CHANGE_SCENE = BASE_URL + "lamps/update-scene";

@Injectable({
  providedIn: 'root'
})
export class HttpService {

  constructor(private http: HttpClient) { }

  getLamps() {
    return this.http.get<Lamp[]>(ALL_LAMPS);
  }

  changeStatusOfLamp(lampId: number, status: boolean) {
    return this.http.post(CHANGE_STATUS, {
      instanceId: lampId,
      status,
    });
  }

  changeBrightnessOfLamp(lampId: number, brightness: number) {
    return this.http.post(CHANGE_BRIGHTNESS, {
      instanceId: Number(lampId),
      brightness: brightness,
    });
  }

  changeScene(scene: Scenes) {
    return this.http.post(CHANGE_SCENE, {
      scene,
    });
  }
}