import { Component, OnInit } from '@angular/core';
import { LampsService } from 'src/app/shared/lamps.service';
import { Lamp } from 'src/app/models/lamp';
import { Scenes } from 'src/app/models/scene';
import { ScenesService } from 'src/app/shared/scenes.service';

@Component({
  selector: 'app-scenes',
  templateUrl: './scenes.component.html',
  styleUrls: ['./scenes.component.scss'],
})
export class ScenesComponent implements OnInit {
  activeScene: Scenes;

  constructor(private sceneService: ScenesService) {}

  ngOnInit(): void {
    this.sceneService.activeScene.subscribe(
      (scene) => (this.activeScene = scene)
    );
  }

  color(scene: Scenes) {
    return this.activeScene === scene ? 'warn' : 'primary';
  }

  onSceneChange(scene: Scenes) {
    this.sceneService.setScene(scene);
  }
}
