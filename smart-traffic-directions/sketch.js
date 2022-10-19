
let sim;
const map = new RoadMap();
const editor = new MapEditor(map)

let algorithm = 'f'
let densityCoeff = 200
let updateInterval = 500
let clearanceCoeff = 0
let velCoeff = 60
let velCoeff2 = 0.001;
function setup() {
	createCanvas(740, 480);
	strokeCap('ROUNDED')

	const mapData = [
		{
			id: '1',
			pos: [116, 221],
			connections: ['a', '2', '4']
		},
		{
			id: '2',
			pos: [211, 403],
			connections: ['1', 'b', '3'],
		},
		{
			id: '3',
			pos: [289, 334],
			connections: ['4', '6', '2'],
		},
		{
			id: '4',
			pos: [199, 189],
			connections: ['1', '3', '5'],
		},
		{
			id: '5',
			pos: [410, 176],
			connections: ['4', '6', 'd'],
		},
		{
			id: '6',
			pos: [452, 260],
			connections: ['d', '5', 'c', '3'],
		},
		{
			id: 'a',
			pos: [43, 161],
			connections: ['1']
		},
		{
			id: 'b',
			pos: [311, 440],
			connections: ['2']
		},
		{
			id: 'c',
			pos: [609, 366],
			connections: ['6']
		},
		{
			id: 'd',
			pos: [579, 167],
			connections: ['5', '6']
		}
	]

	for (const intersection of mapData) {
		map.createIntersection(intersection.id, createVector(intersection.pos[0], intersection.pos[1]), intersection.connections)
	}

	sim = new Simulator(map)


	// map.createIntersection('a', createVector(100, 100), [], {})
	// map.createIntersection('n', createVector(200, 150), ['a'], {})
	// map.createIntersection('c', createVector(250, 250), ['n'],
	// 	{ 'n': [createVector(180, 180), createVector(230, 200)] }
	// )

	// map.createIntersection('b', createVector(300, 300), ['c'])
	// map.createIntersection('d', createVector(100, 300), ['c', 'n'])
	// map.createIntersection('e', createVector(10, 350), ['d', 'b'])
	// map.createIntersection('f', createVector(100, 30), ['n', 'a', 'b'])
	// map.createIntersection('g', createVector(200, 380), ['n', 'a', 'b'])
	// map.createIntersection('h', createVector(380, 350), ['g', 'f', 'a', 'b'])
	// map.createIntersection('i', createVector(280, 350), ['g', 'f', 'a', 'b'])

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