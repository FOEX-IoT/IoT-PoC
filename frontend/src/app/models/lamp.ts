export type Status = 'on' | 'off';

export interface Lamp {
  instanceId: number;
  name: string;
  brightness: number;
  status: boolean;
}