/* tslint:disable */
/* eslint-disable */
/**
*/
export function greet(): void;
/**
*/
export class Simulator {
  free(): void;
/**
* @returns {Simulator}
*/
  static new(): Simulator;
/**
*/
  spawn_vehicles(): void;
/**
* @param {number} scale
* @param {number} density_coeff
* @param {number} vel_coeff
*/
  tick(scale: number, density_coeff: number, vel_coeff: number): void;
/**
* @returns {number}
*/
  get_vehicle_render_buff_ptr(): number;
/**
* @returns {number}
*/
  get_vehicle_render_buff_len(): number;
/**
* @returns {Uint32Array}
*/
  get_map_render_data(): Uint32Array;
/**
*/
  stats: StatsManager;
}
/**
*/
export class StatsManager {
  free(): void;
/**
*/
  completed_vehicle_count: number;
}
