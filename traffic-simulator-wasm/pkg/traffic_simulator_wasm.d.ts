/* tslint:disable */
/* eslint-disable */
/**
*/
export class Simulator {
  free(): void;
/**
* @returns {Simulator}
*/
  static new(): Simulator;
/**
* @param {number} n
*/
  spawn_vehicles(n: number): void;
/**
* @param {number} scale
* @param {number} density_coeff
* @param {number} vel_coeff
*/
  tick(scale: number, density_coeff: number, vel_coeff: number): void;
/**
* @param {number} id
* @param {number} x
* @param {number} y
* @param {number | undefined} weight
*/
  create_intersection(id: number, x: number, y: number, weight?: number): void;
/**
* @param {number} n1
* @param {number} n2
*/
  create_road(n1: number, n2: number): void;
/**
* @param {number} n1
* @param {number} n2
*/
  delete_road(n1: number, n2: number): void;
/**
* @param {number} id
*/
  delete_intersection(id: number): void;
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
  avg_flux: number;
/**
*/
  avg_vel: number;
/**
*/
  completed_vehicle_count: number;
/**
*/
  flux_avg_clear_threshold: number;
/**
*/
  vehicle_on_road: number;
/**
*/
  vel_avg_clear_threshold: number;
}
