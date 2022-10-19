
let sim;
const map = new RoadMap();
const editor = new MapEditor(map)

let algorithm = 'f'
let densityCoeff = 200
let updateInterval = 500
let clearanceCoeff = 0
let velCoeff = 100
let velCoeff2 = 0.0005;
let brakingProbability = 0.0001
function setup() {

	createCanvas(740, 480);
	strokeCap('ROUNDED')

	// for (const intersection of mapData) {
	// 	map.createIntersection(intersection.id, createVector(intersection.pos[0], intersection.pos[1]), intersection.connections)
	// }

	map.create_from({
		intersections: [
			{
				id: 'a',
				pos: [100, 100]
			},
			{
				id: 'b',
				pos: [300, 300]
			}
		],
		roads: [
			{
				n1: 'a',
				n2: 'b'
			}
		]
	})


	sim = new Simulator(map)

}

function draw() {
	background(55);
	// map.render()
	editor.show()
	sim.run(editor, map)
	map.update(sim)
}

function mouseClicked() {
	editor.onMouseClick()
}

function keyPressed() {
	editor.onKeyPress()
}

function keyReleased() {
	editor.onKeyRelease()
}