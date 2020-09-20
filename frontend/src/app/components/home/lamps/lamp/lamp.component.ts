import { Component, OnInit, Input } from '@angular/core';
import { Status, Lamp } from 'src/app/models/lamp';
import { LampsService } from 'src/app/shared/lamps.service';
import { ScenesService } from 'src/app/shared/scenes.service';

@Component({
  selector: 'app-lamp',
  templateUrl: './lamp.component.html',
  styleUrls: ['./lamp.component.scss'],
})
export class LampComponent implements OnInit {
  @Input('lamp')
  lamp: Lamp;

  isActive: number = 0;

  // @Input('brightness')
  // brightness: number;

  // @Input('status')
  // status: Status;

  // @Input('id')
  // id: number;

  constructor(
    private lampsService: LampsService,
    private scenesService: ScenesService
  ) {}

  ngOnInit(): void {
    this.lampsService.activeLamp.subscribe(
      (active) => (this.isActive = active)
    );
  }

  changeActive() {
    this.lampsService.activeLamp.next(this.lamp.instanceId);
  }

  changeClosed() {
    if (this.isActive === this.lamp.instanceId) {
      this.lampsService.activeLamp.next(-1);
    }
  }

  changeBrightness(event: Event) {
    this.lampsService.changeBrightness(event.value, this.lamp.instanceId)
  }

  toggleStatus() {
    this.lampsService.toggleStatus(this.lamp.instanceId);
  }
}