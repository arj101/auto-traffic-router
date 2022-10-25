import * as wasm from "traffic-simulator-wasm";
import { memory } from "traffic-simulator-wasm/traffic_simulator_wasm_bg";


const sim = wasm.Simulator.new();

const mapRenderData = sim.get_map_render_data();

const canvas = document.getElementsByTagName('canvas')[0]
// canvas.width = window.innerWidth - 5;
// canvas.height = window.innerHeight - 5;

// window.onresize = () => {
//     canvas.width = window.innerWidth - 5;
//     canvas.height = window.innerHeight - 5;
// }

const ctx = canvas.getContext('2d')

sim.spawn_vehicles();

let velCoeff = 0;
let densityCoeff = 0;

const densitySlider = document.getElementById('density')
const velocitySlider = document.getElementById('velocity')
velCoeff = parseFloat(velocitySlider.value)
densityCoeff = parseFloat(densitySlider.value);

densitySlider.onchange = () => {
    console.log(densitySlider.value)
    densityCoeff = parseFloat(densitySlider.value);

}


velocitySlider.onchange = () => {
    console.log(velocitySlider.value)
    velCoeff = parseFloat(velocitySlider.value)

}

const renderLoop = () => {
    if (Math.random() < 0.02) { sim.spawn_vehicles() }

    ctx.clearRect(0, 0, canvas.width, canvas.height)
    ctx.fillStyle = 'rgb(30, 30,30)'
    ctx.fillRect(0, 0, canvas.width, canvas.height)
    ctx.fill()
    ctx.fillStyle = 'rgba(10, 10, 10, 0.5)'
    ctx.arc(canvas.width / 2, canvas.height / 2, 5, 0, 2 * Math.PI)
    ctx.fill()

    ctx.lineWidth = 10
    ctx.lineCap = 'round'
    ctx.strokeStyle = 'rgb(100, 100, 100)'
    ctx.fillStyle = 'rgb(150, 150, 150)'

    let i = 0;
    while (i < mapRenderData.length) {
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

        i += 6;
    }


    sim.tick(10, densityCoeff, velCoeff)

    const buffLen = sim.get_vehicle_render_buff_len();
    const buffPtr = sim.get_vehicle_render_buff_ptr();
    const vehicleRenderData = new Float32Array(memory.buffer, buffPtr, buffLen);
    i = 0;
    ctx.fillStyle = 'rgb(20, 150, 100)'
    ctx.lineWidth = 1;
    ctx.strokeStyle = 'rgb(255, 50, 50)'
    while (i < buffLen) {
        ctx.beginPath()
        ctx.arc(vehicleRenderData[i + 1], vehicleRenderData[i + 2], 4, 0, Math.PI * 2)
        ctx.fill()
        ctx.stroke()
        ctx.closePath()
        i += 3
    }

    ctx.fillStyle = 'rgb(255, 255, 255)'
    ctx.font = '14px Fira Code'
    ctx.fillText(`vehicle count: ${sim.stats.completed_vehicle_count}`, 20, 20);
    ctx.fillText(`flux: ${sim.stats.avg_flux.toFixed(5)}`, 20, 40);
    ctx.fillText(`speed: ${sim.stats.avg_vel.toFixed(5)}`, 20, 60);
    ctx.fill()

    requestAnimationFrame(renderLoop)

}

requestAnimationFrame(renderLoop)