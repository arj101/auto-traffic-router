import * as wasm from "traffic-simulator-wasm";
import { memory } from "traffic-simulator-wasm/traffic_simulator_wasm_bg";
import mapData from "./map-data"


const sim = wasm.Simulator.new();

const nameMap = new Map();
const idMap = new Map();

const createMap = (mapData) => {
    let idCounter = 0;
    for (const intersection of mapData.intersections) {
        let name;
        let id = idCounter;
        let x, y;
        let weight;

        if (Array.isArray(intersection)) {
            name = intersection[0]
            x = intersection[1]
            y = intersection[2]
            weight = intersection[3]
        } else {
            name = intersection.id;
            x = intersection.pos[0]
            y = intersection.pos[1]
            weight = intersection.weight;
        }

        if (!(typeof x == 'number' && typeof y == 'number' && typeof name == 'string')) continue;

        nameMap.set(name, id)
        idMap.set(id, name)


        sim.create_intersection(id, x, y, weight)
        idCounter += 1;
    }

    for (const road of mapData.roads) {
        let n1, n2;
        if (Array.isArray(road)) {
            n1 = road[0]
            n2 = road[1]
        } else {
            n1 = road.n1;
            n2 = road.n2;
        }
        if (!(typeof n1 == 'string' && typeof n2 == 'string')) continue;
        if (nameMap.has(n1) && nameMap.has(n2)) {
            sim.create_road(nameMap.get(n1), nameMap.get(n2))
        }
    }

}

createMap(mapData)


const mapRenderData = sim.get_map_render_data();

const canvas = document.getElementsByTagName('canvas')[0]
const ctx = canvas.getContext('2d', { alpha: false })


const dpr = window.devicePixelRatio;
let rect = canvas.getBoundingClientRect();
canvas.width = rect.width * dpr;
canvas.height = rect.height * dpr;

ctx.scale(dpr, dpr)
canvas.style.width = `${rect.width}px`
canvas.style.height = `${rect.height}px`


let velCoeff = 0;
let densityCoeff = 0;
let spawnRate = 0;

const densitySlider = document.getElementById('density')
const velocitySlider = document.getElementById('velocity')
const spawnRateSlider = document.getElementById('spawn-rate')
const spawnRateSpan = document.getElementById('spawn-rate-display')

const vehilceCountP = document.getElementById('vehicle-count')
const fluxP = document.getElementById('flux')
const speedP = document.getElementById('speed')

velCoeff = parseFloat(velocitySlider.value)
densityCoeff = parseFloat(densitySlider.value);
spawnRate = parseFloat(spawnRateSlider.value) / 200
spawnRateSpan.textContent = `${spawnRate.toFixed(3)}`

densitySlider.onchange = () => {
    densityCoeff = parseFloat(densitySlider.value);

}

velocitySlider.onchange = () => {
    velCoeff = parseFloat(velocitySlider.value)

}

spawnRateSlider.onchange = () => {
    spawnRate = parseFloat(spawnRateSlider.value) / 200
    spawnRateSpan.textContent = `${spawnRate.toFixed(3)}`
}

const renderLoop = () => {
    if (Math.random() < spawnRate) { sim.spawn_vehicles(5) }

    ctx.beginPath();


    ctx.fillStyle = 'rgb(30, 30,30)'
    ctx.fillRect(0, 0, canvas.width, canvas.height)
    ctx.fill()
    ctx.lineWidth = 10
    ctx.lineCap = 'round'
    ctx.strokeStyle = 'rgb(100, 100, 100)'
    ctx.fillStyle = 'rgb(150, 150, 150)'


    for (let i = 0; i < mapRenderData.length; i += 6) {
        ctx.beginPath();
        const [x1, y1, x2, y2] = mapRenderData.slice(i + 2, i + 6);
        ctx.moveTo(x1, y1)
        ctx.lineTo(x2, y2)
        ctx.stroke();
        ctx.closePath();

        ctx.beginPath()
        ctx.arc(x1, y1, 6, 0, Math.PI * 2)
        ctx.closePath()
        ctx.fill()
        ctx.beginPath()
        ctx.arc(x2, y2, 6, 0, Math.PI * 2)
        ctx.closePath()
        ctx.fill()

    }


    sim.tick(10, densityCoeff, velCoeff)

    const buffLen = sim.get_vehicle_render_buff_len();
    const buffPtr = sim.get_vehicle_render_buff_ptr();
    const vehicleRenderData = new Float32Array(memory.buffer, buffPtr, buffLen);
    ctx.fillStyle = 'rgb(20, 150, 100)'
    ctx.lineWidth = 1;
    ctx.strokeStyle = 'rgb(255, 50, 50)'
    for (let i = 0; i < buffLen; i += 3) {
        ctx.beginPath()
        ctx.arc(vehicleRenderData[i + 1], vehicleRenderData[i + 2], 4, 0, Math.PI * 2)
        ctx.stroke()
        ctx.closePath()
    }

    ctx.fillStyle = 'rgb(255, 255, 255)'
    ctx.font = '14px Fira Code'
    vehilceCountP.textContent = `vehicle count: ${sim.stats.completed_vehicle_count}`;
    fluxP.textContent = `flux: ${sim.stats.avg_flux.toFixed(5)}`;
    speedP.textContent = `speed: ${sim.stats.avg_vel.toFixed(5)}`;
    ctx.fill()

    ctx.closePath()

    requestAnimationFrame(renderLoop)

}

// setInterval()
requestAnimationFrame(renderLoop)
