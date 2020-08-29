import { Injectable } from '@angular/core';
import { LampsService } from './lamps.service';
import { BehaviorSubject } from 'rxjs';
import { Scenes } from '../models/scene';

@Injectable({
  providedIn: 'root',
})
export class ScenesService {
  activeScene = new BehaviorSubject<Scenes>('CUSTOM');

  constructor(private lampsService: LampsService) {}

  setScene(scene: Scenes) {
    this.activeScene.next(scene);
  }
}
