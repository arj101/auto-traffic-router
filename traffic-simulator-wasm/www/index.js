import * as wasm from "traffic-simulator-wasm";

wasm.greet()

const sim = wasm.Simulator.new();

const canvas = document.getElementsByTagName('canvas')[0]
canvas.width = window.innerWidth - 5;
canvas.height = window.innerHeight - 5;

window.onresize = () => {
    canvas.width = window.innerWidth - 5;
    canvas.height = window.innerHeight - 5;
}

const ctx = canvas.getContext('2d')


const renderLoop = () => {
    requestAnimationFrame(renderLoop)

    ctx.clearRect(0, 0, canvas.width, canvas.height)
    ctx.fillStyle = 'rgb(30, 30,30)'
    ctx.fillRect(0, 0, canvas.width, canvas.height)
    ctx.fill()
    ctx.fillStyle = 'rgba(10, 10, 10, 0.5)'
    ctx.arc(canvas.width / 2, canvas.height / 2, 5, 0, 2 * Math.PI)
    ctx.fill()
}

renderLoop()