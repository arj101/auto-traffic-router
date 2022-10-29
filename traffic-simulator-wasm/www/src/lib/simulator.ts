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
    spritePool: Array<PIXI.Sprite>;

    spawnProbability: number;
    densityCoeff: number;
    velocityCoeff: number;
    timeScale: number;

    constructor(canvas: HTMLCanvasElement, map = map4) {
        this.nameMap = new Map();
        this.idMap = new Map();
        this.sim = wasm.Simulator.new();

        const dpi = window.devicePixelRatio;
        const rect = canvas.getBoundingClientRect();
        canvas.width = rect.width * dpi;
        canvas.height = rect.height * dpi;
        canvas.style.width = `${rect.width}px`;
        canvas.style.height = `${rect.height}px`;
        this.pixiApp = new PIXI.Application({
            width: 640,
            height: 480,
            view: canvas,
            backgroundColor: 0x202020,
            antialias: true,
            resolution: window.devicePixelRatio,
        });
        window.addEventListener("resize", this.pixiApp.resize);
        this.pixiApp.renderer.plugins.interaction.autoPreventDefault = false;
        this.pixiApp.renderer.view.style.touchAction = "auto";
        this.mapGraphics = new PIXI.Graphics();
        this.spawnProbability = 0;
        this.densityCoeff = 0;
        this.velocityCoeff = 0;
        this.timeScale = 1;

        this.pixiApp.stage.addChild(this.mapGraphics);

        if (!this.createMap(map)) {
            console.warn("Parsing map data failed.");
        }
        this.drawMap();
        this.createCircleTexture();

        this.vehicleMap = new Map();
        this.spritePool = new Array(50).fill(
            PIXI.Sprite.from(this.vehicleTexture)
        );

        this.stop();

        this.pixiApp.ticker.add(() => this.tick());
    }

    reinstantiateWithMap(map) {
        this.stop();

        this.sim.free();
        this.sim = wasm.Simulator.new();

        this.idMap.clear();
        this.nameMap.clear();
        for (const [_, s] of this.vehicleMap) {
            this.pixiApp.stage.removeChild(s);
            this.spritePool.push(s);
        }
        this.vehicleMap.clear();

        if (!this.createMap(map)) {
            console.warn("Parsing map data failed.");
            return;
        }
        this.drawMap();

        this.start();
    }

    stop() {
        this.running = false;
        this.pixiApp.stop();
    }

    start() {
        this.running = true;
        this.pixiApp.start();
    }

    tick() {
        if (Math.random() < this.spawnProbability) this.sim.spawn_vehicles(2);

        this.sim.tick(
            10 * this.timeScale,
            this.densityCoeff,
            this.velocityCoeff
        );
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
                sprite = this.spritePool.pop();
                if (!sprite) sprite = PIXI.Sprite.from(this.vehicleTexture);
                this.pixiApp.stage.addChild(sprite);
            }

            sprite.x = x - sprite.width / 2;
            sprite.y = y - sprite.height / 2;
            newVehicleMap.set(id, sprite);
        }
        for (const [_, s] of this.vehicleMap) {
            this.pixiApp.stage.removeChild(s);
            this.spritePool.push(s);
        }
        this.vehicleMap = newVehicleMap;
    }

    drawMap() {
        this.mapGraphics.clear();
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
    }

    createCircleTexture() {
        const circle = new PIXI.Graphics();
        circle.beginFill(0xffffff, 0.2);
        circle.drawCircle(4, 4, 4);
        circle.endFill();
        this.vehicleTexture = this.pixiApp.renderer.generateTexture(circle, {
            resolution: window.devicePixelRatio,
        });
    }

    createMap(mapData) {
        if (!mapData.intersections) return false;
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
        return true;
    }
}
