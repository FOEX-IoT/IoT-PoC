import { Component, OnInit } from '@angular/core';
import { LampsService } from '../../../shared/lamps.service';
import { Lamp } from 'src/app/models/lamp';

@Component({
  selector: 'app-lamps',
  templateUrl: './lamps.component.html',
  styleUrls: ['./lamps.component.scss'],
})
export class LampsComponent implements OnInit {
  lamps: Lamp[];
  activeLamp: number;
  constructor(private lampsService: LampsService) {}

  ngOnInit(): void {
    this.lampsService.lamps.subscribe((lamps) => {
      this.lamps = lamps;
    });
    this.lampsService.activeLamp.subscribe((id) => {
      this.activeLamp = id;
    });
  }

  changeLamp = (id: number) => {
    this.lampsService.activeLamp.next(id);
  };
}
