/* tslint:disable */
/* eslint-disable */
export const memory: WebAssembly.Memory;
export function __wbg_statsmanager_free(a: number): void;
export function __wbg_get_statsmanager_completed_vehicle_count(a: number): number;
export function __wbg_set_statsmanager_completed_vehicle_count(a: number, b: number): void;
export function __wbg_get_statsmanager_avg_flux(a: number): number;
export function __wbg_set_statsmanager_avg_flux(a: number, b: number): void;
export function __wbg_get_statsmanager_flux_avg_clear_threshold(a: number): number;
export function __wbg_set_statsmanager_flux_avg_clear_threshold(a: number, b: number): void;
export function __wbg_get_statsmanager_avg_vel(a: number): number;
export function __wbg_set_statsmanager_avg_vel(a: number, b: number): void;
export function __wbg_get_statsmanager_vel_avg_clear_threshold(a: number): number;
export function __wbg_set_statsmanager_vel_avg_clear_threshold(a: number, b: number): void;
export function __wbg_simulator_free(a: number): void;
export function __wbg_get_simulator_stats(a: number): number;
export function __wbg_set_simulator_stats(a: number, b: number): void;
export function simulator_new(): number;
export function simulator_spawn_vehicles(a: number): void;
export function simulator_tick(a: number, b: number, c: number, d: number): void;
export function simulator_get_vehicle_render_buff_ptr(a: number): number;
export function simulator_get_vehicle_render_buff_len(a: number): number;
export function simulator_get_map_render_data(a: number): number;
export function __wbindgen_free(a: number, b: number): void;
export function __wbindgen_malloc(a: number): number;
export function __wbindgen_realloc(a: number, b: number, c: number): number;
export function __wbindgen_exn_store(a: number): void;