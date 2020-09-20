import { Component, OnInit } from '@angular/core';
import { TemperatureService } from '../../../shared/temperature.service';

@Component({
  selector: 'app-header',
  templateUrl: './header.component.html',
  styleUrls: ['./header.component.scss'],
})
export class HeaderComponent implements OnInit {
  temperature: number;

  constructor(private temperatureService: TemperatureService) {}

  ngOnInit(): void {
    this.temperatureService.getCurrentTemperature().subscribe((temp) => {
      this.temperature = temp;
    });
    this.getCurrentGreeting();
  }

  getCurrentGreeting(): string {
    let date = new Date();
    let hours = date.getHours();
    if (hours >= 0 && hours <= 12) {
      return 'Guten Morgen!';
    } else if (hours > 12 && hours <= 19) {
      return 'Guten Tag!';
    } else {
      return 'Guten Abend!';
    }
  }
}
