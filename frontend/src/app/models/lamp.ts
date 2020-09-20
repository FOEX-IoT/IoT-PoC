export type Status = 'on' | 'off';

export interface Lamp {
  id: number;
  name: string;
  brightness: number;
  status: Status;
}
