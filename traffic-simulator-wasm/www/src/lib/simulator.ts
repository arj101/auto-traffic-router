import * as wasm from "traffic-simulator-wasm";
import { memory } from "traffic-simulator-wasm/traffic_simulator_wasm_bg.wasm";
import * as PIXI from "pixi.js";
import { map4 } from "./map-data";

export class Simulator {
    idMap: Map<number, string>;
    nameMap: Map<string, number>;
    sim: wasm.Simulator;
    pixiApp: PIXI.Application;
    mapGraphics: PIXI.Graphics;
    running: boolean;
    mapRenderData: Uint32Array;
    vehicleTexture: PIXI.RenderTexture;
    vehicleMap: Map<number, PIXI.Sprite>;

    constructor(canvas: HTMLCanvasElement) {
        this.nameMap = new Map();
        this.idMap = new Map();
        this.sim = wasm.Simulator.new();
        this.createMap(map4);

        const dpi = window.devicePixelRatio;
        const rect = canvas.getBoundingClientRect();
        canvas.width = rect.width * dpi;
        canvas.height = rect.height * dpi;
        canvas.style.width = `${rect.width}px`;
        canvas.style.height = `${rect.height}px`;
        this.pixiApp = new PIXI.Application({
      
            view: canvas,
            backgroundColor: 0x202020,
            antialias: true,
            resolution: window.devicePixelRatio,
        });
        this.mapGraphics = new PIXI.Graphics();

        for (let i = 0; i < this.mapRenderData.length; i += 6) {
            const [x1, y1, x2, y2] = this.mapRenderData.slice(i + 2, i + 6);

            this.mapGraphics.lineStyle({ width: 10, color: 0x404040 });
            this.mapGraphics.moveTo(x1, y1);
            this.mapGraphics.lineTo(x2, y2);
            this.mapGraphics.lineStyle({ width: 0 });

            this.mapGraphics.beginFill(0xaaaa55, 1);
            this.mapGraphics.drawCircle(x1, y1, 8);
            this.mapGraphics.endFill();

            this.mapGraphics.beginFill(0xaaaa55, 1);
            this.mapGraphics.drawCircle(x2, y2, 8);
            this.mapGraphics.endFill();
        }
        this.pixiApp.stage.addChild(this.mapGraphics);

        // const renderer = PIXI.autoDetectRenderer();

        const circle = new PIXI.Graphics();

        circle.lineStyle({ width: 2, color: 0xdeadbeef });
        circle.drawCircle(4, 4, 4);
        this.vehicleTexture = this.pixiApp.renderer.generateTexture(circle, {
            resolution: dpi,
        });

        this.vehicleMap = new Map();

        this.running = false;
        this.pixiApp.start();
        this.pixiApp.ticker.add(() => {
            if (Math.random() < 0.1) this.sim.spawn_vehicles(10);

            this.sim.tick(10, 20, 20);
            const buffLen = this.sim.get_vehicle_render_buff_len();
            const buffPtr = this.sim.get_vehicle_render_buff_ptr();
            const vehicleRenderData = new Float32Array(
                memory.buffer,
                buffPtr,
                buffLen
            );
            const newVehicleMap = new Map();
            for (let i = 0; i < buffLen; i += 3) {
                const id = vehicleRenderData[i];
                const x = vehicleRenderData[i + 1];
                const y = vehicleRenderData[i + 2];
                let sprite;
                if (this.vehicleMap.has(id)) {
                    sprite = this.vehicleMap.get(id);
                    this.vehicleMap.delete(id);
                } else {
                    sprite = PIXI.Sprite.from(this.vehicleTexture);
                    this.pixiApp.stage.addChild(sprite);
                }

                sprite.x = x - sprite.width / 2;
                sprite.y = y - sprite.height / 2;
                newVehicleMap.set(id, sprite);
            }
            for (const [_, s] of this.vehicleMap) {
                this.pixiApp.stage.removeChild(s);
                s.destroy();
            }
            this.vehicleMap = newVehicleMap;
        });
    }

    createMap(mapData) {
        let idCounter = 0;
        for (const intersection of mapData.intersections) {
            let name;
            let id = idCounter;
            let x, y;
            let weight;

            if (Array.isArray(intersection)) {
                name = intersection[0];
                x = intersection[1];
                y = intersection[2];
                weight = intersection[3];
            } else {
                name = intersection.id;
                x = intersection.pos[0];
                y = intersection.pos[1];
                weight = intersection.weight;
            }

            if (
                !(
                    typeof x == "number" &&
                    typeof y == "number" &&
                    typeof name == "string"
                )
            )
                continue;

            this.nameMap.set(name, id);
            this.idMap.set(id, name);

            this.sim.create_intersection(id, x, y, weight);
            idCounter += 1;
        }

        for (const road of mapData.roads) {
            let n1, n2;
            if (Array.isArray(road)) {
                n1 = road[0];
                n2 = road[1];
            } else {
                n1 = road.n1;
                n2 = road.n2;
            }
            if (!(typeof n1 == "string" && typeof n2 == "string")) continue;
            if (this.nameMap.has(n1) && this.nameMap.has(n2)) {
                this.sim.create_road(
                    this.nameMap.get(n1),
                    this.nameMap.get(n2)
                );
            }
        }

        this.mapRenderData = this.sim.get_map_render_data();
    }
}
