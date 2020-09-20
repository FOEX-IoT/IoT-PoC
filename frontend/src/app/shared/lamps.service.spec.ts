import { TestBed } from '@angular/core/testing';

import { LampsService } from './lamps.service';

describe('LampsService', () => {
  let service: LampsService;

  beforeEach(() => {
    TestBed.configureTestingModule({});
    service = TestBed.inject(LampsService);
  });

  it('should be created', () => {
    expect(service).toBeTruthy();
  });
});
