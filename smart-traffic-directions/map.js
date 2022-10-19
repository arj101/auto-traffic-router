
class TrafficMap {
	constructor() {
		this.nodes = new Map();
		this.edges = new Map();
		this.idGen = new IdGenerator();
		this.costs = new Map();
	}


	addNode(nodeId, pos, adjacentNodes = [], onEdgeAdd = () => { }) {
		const node = new Node(nodeId, pos);

		for (const adjacentId of adjacentNodes) {
			if (!this.nodes.has(adjacentId)) continue;
			const adjacent = this.nodes.get(adjacentId);
			const edgeId = `${nodeId}-${adjacent.id}`;
			this.edges.set(edgeId, new Edge(edgeId, pos, adjacent.pos, node, adjacent))
			onEdgeAdd(edgeId)
			node.edges.push(edgeId)
			adjacent.edges.push(edgeId)
		}

		this.nodes.set(nodeId, node);
	}

	render() {
		this.edges.forEach(edge => {
			stroke(255)
			strokeWeight(2)
			line(edge.p1.x, edge.p1.y, edge.p2.x, edge.p2.y)
			textSize(14);
			fill(255)
			noStroke()
			text(edge.id, edge.p1.x / 2 + edge.p2.x / 2, edge.p1.y / 2 + edge.p2.y / 2)
		})
		this.nodes.forEach(node => {
			stroke(255)
			strokeWeight(5)
			point(node.pos.x, node.pos.y)
		})
	}
}

class Edge {
	constructor(id, p1, p2, node1, node2, cost = 1.0) {
		this.id = id;
		this.p1 = p1;
		this.p2 = p2;
		this.node1 = node1;
		this.node2 = node2;
		this.cost = cost;
	}
}

class Node {
	constructor(id, pos, edges = []) {
		this.pos = pos;
		this.id = id;
		this.edges = edges;
	}
}


class IdGenerator {
	constructor() {
		this.curr = 0;
	}
	generate() {
		const id = this.curr;
		this.curr += 1;
		return id;
	}
}

class Road {
	constructor(p1, p2, node1Id, node2Id, vertices = []) {
		this.node1Id = node1Id;
		this.node2Id = node2Id;
		this.p1 = p1;
		this.p2 = p2;
		this.vertices = [p1, ...vertices, p2];
		this.pathSegments = []
		this.pathLength = this.getPathLength()

		let prevDist = 0;
		for (let i = 0; i < this.vertices.length - 1; i++) {
			const v1 = this.vertices[i]
			const v2 = this.vertices[i + 1]
			const dv1v2 = dist(v1.x, v1.y, v2.x, v2.y)
			this.pathSegments.push([prevDist / this.pathLength, (prevDist + dv1v2) / this.pathLength])
			prevDist += dv1v2
		}

		this.costFwd = this.pathLength;
		this.costBck = this.pathLength;

		this.laneFwd = new Map()
		this.laneBck = new Map()

		this.lastFwdLaneVehicle = null
		this.lastBckLaneVehicle = null

		const updateFn = () => {
			this.dCostFwd = this.dynamicCost(this.node1Id)
			this.dCostBck = this.dynamicCost(this.node2Id)
			setTimeout(updateFn, updateInterval)
		}

		updateFn()

	}

	id() {
		return `${this.node1Id}-${this.node2Id}`
	}

	onSameLane(vehicleId, otherVehicleId) {
		if (this.laneFwd.has(vehicleId)) return this.laneFwd.has(otherVehicleId)
		return this.laneBck.has(vehicleId) && this.laneBck.has(otherVehicleId)
	}


	enter(vehicleId, nodeId, sim) {
		if (nodeId == this.node1Id) {
			this.laneFwd.set(vehicleId, 0)
			const r = { infront: this.lastFwdLaneVehicle, pos: 0, dir: 1, pathSegment: 0 }
			this.lastFwdLaneVehicle = vehicleId
			return r
		} else if (nodeId == this.node2Id) {
			this.laneBck.set(vehicleId, this.pathLength)
			const r = { infront: this.lastBckLaneVehicle, pos: this.pathLength, dir: -1, pathSegment: this.pathSegments.length - 1 }
			this.lastBckLaneVehicle = vehicleId
			return r
		}
		console.warn(`Unknown entry point for vehicle '${vehicleId}'`)
		return {}
	}

	update(sim) {
		const fwdLane = []
		const bckLane = []
		for (const [id, pos] of this.laneFwd) fwdLane.push([id, pos])
		for (const [id, pos] of this.laneBck) bckLane.push([id, pos])

		fwdLane.sort((a, b) => a[1] - b[1])
		bckLane.sort((a, b) => b[1] - a[1])


		for (let idx = 0; idx < fwdLane.length; idx++) {
			if (sim.vehicles.get(fwdLane[idx][0])) sim.vehicles.get(fwdLane[idx][0]).setInfront(fwdLane[idx + 1]?.[0], sim)

		}

		for (let idx = 0; idx < bckLane.length; idx++) {
			if (sim.vehicles.get(bckLane[idx][0])) sim.vehicles.get(bckLane[idx][0]).setInfront(bckLane[idx + 1]?.[0], sim)
		}

		// if (fwdLane.length > 0) console.log(fwdLane, this.id())
	}

	updatePos(vehicleId, pos, pathSegment) {
		let newPathSegment = pathSegment
		if (this.laneBck.has(vehicleId)) {
			this.laneBck.set(vehicleId, constrain(pos, 0, this.pathLength))

			if (pos / this.pathLength <= this.pathSegments[pathSegment][0])
				newPathSegment -= 1

			if (pos <= 0) {
				this.laneBck.delete(vehicleId)
				return { exitNode: this.node1Id, newPathSegment, }
			}

			return { newPathSegment }

		}
		if (this.laneFwd.has(vehicleId)) {
			this.laneFwd.set(vehicleId, constrain(pos, 0, this.pathLength))

			if (pos / this.pathLength >= this.pathSegments[pathSegment][1])
				newPathSegment += 1

			if (pos >= this.pathLength) {
				this.laneFwd.delete(vehicleId)

				return { exitNode: this.node2Id, newPathSegment }
			}
			return { newPathSegment }

		}
		// console.log(`Unknown vehicle ${vehicleId} on ${this.node1Id}-${this.node2Id}`)
		return {}
	}

	setCost(cost) {
		this.cost = cost
	}

	cost(nodeInitial, algorithm = 'f') {
		if (nodeInitial == this.node1Id) return this.costFwd + (algorithm == 'f' ? this.dCostFwd : 0)
		else return this.costBck + (algorithm == 'f' ? this.dCostBck : 0)
	}

	sortVehicles() {
		for (const [id, pos] of this.laneFwd.entries()) {

		}
	}

	dynamicCost(nodeInitial) {
		if (algorithm == 's') return 0

		if (nodeInitial == this.node1Id) {
			let invDistSum = 0
			let nClearance = 0
			let avgVel = 0
			let nVel = 0;
			this.laneFwd.forEach((_, vid) => {
				const v = sim.vehicles.get(vid)
				if (v?.getPos(5)) {
					avgVel += (Math.abs((v.pos - v.getPos(5)) / 20)) / (deltaTime / 1000)
					nVel += 1
				}
				if (v?.vehicleInfront) {
					invDistSum += 1 / (10e-10 + Math.abs(v.pos - v.vehicleInfront.pos))
					nClearance += 1
				}
			})

			if (nVel > 0) avgVel /= nVel

			let cost = 0
			cost +=
				densityCoeff * this.laneFwd.size / this.pathLength
			if (nVel > 0) cost += velCoeff * (1 / (10e-10 + avgVel))
			cost += densityCoeff * this.laneFwd.size / this.pathLength * clearanceCoeff * ((invDistSum != 0 && invDistSum != Infinity) ? invDistSum / nClearance : 0)
			return (cost
			)
		}
		else {
			let invDistSum = 0
			let nClearance = 0
			let avgVel = 0
			let nVel = 0
			this.laneBck.forEach((_, vid) => {
				const v = sim.vehicles.get(vid)
				if (v?.getPos(5)) {
					avgVel += (Math.abs((v.pos - v.getPos(5)) / 20)) / (deltaTime / 1000)
					nVel += 1
				}
				if (v?.vehicleInfront) {
					invDistSum += 1 / (10e-10 + Math.abs(v.pos - v.vehicleInfront.pos))
					nClearance += 1
				}
			})

			if (avgVel > 0)
				avgVel /= nVel

			let cost = 0

			cost += densityCoeff * this.laneBck.size / this.pathLength;
			if (nVel > 0) cost += velCoeff * (1 / (10e-10 + avgVel))
			densityCoeff * this.laneBck.size / this.pathLength * clearanceCoeff * ((invDistSum != 0 && invDistSum != Infinity) ? invDistSum / nClearance : 0)
			return (
				cost
			)
		}
	}

	getPathLength() {
		let length = 0;
		let prevV = this.vertices[0]
		for (const v of this.vertices) {
			length += dist(v.x, v.y, prevV.x, prevV.y)
			prevV = v
		}
		return length
	}

	setVertices(vertices = []) {
		this.vertices = [this.p1, ...vertices, this.p2];
	}

	p1() {
		this.vertices[0]
	}

	p2() {
		this.vertices[this.vertices.length - 1]
	}

	distanceToPoint(p) {
		let distShortest = Infinity;
		let pointShortest = createVector(0, 0)
		for (let i = 0; i < this.vertices.length - 1; i++) {
			const p1 = this.vertices[i]
			const p2 = this.vertices[i + 1]
			const d1 = p5.Vector.sub(p2, p1)
			const d2 = p5.Vector.sub(p, p1)
			const l1 = d1.mag()

			const dotp = constrain(d2.dot(d1.normalize()), 0, l1)
			const point = p5.Vector.add(p1, d1.mult(dotp))

			const d = dist(point.x, point.y, p.x, p.y)
			if (d < distShortest) {
				distShortest = d
				pointShortest = point
			}
		}

		return [distShortest, pointShortest]

	}
}

class Intersection {
	constructor(id, pos, connections) {
		this.id = id;
		this.pos = pos;
		this.connections = connections;
		this.costs = new Map();
		this.roads = [];
	}
}


class RoadMap {
	constructor(trafficMap = new TrafficMap()) {
		this.trafficMap = trafficMap;
		this.roads = new Map();
		this.intersections = new Map();
	}

	clearCache() {
		this.intersections.forEach(i => i.costs.clear())
	}

	createIntersection(id, pos, connections = [], edgePath = {}) {
		this.trafficMap.addNode(id, pos, connections)
		const intersection = new Intersection(id, pos, connections)
		for (const nodeId of connections) {
			if (!this.trafficMap.nodes.has(nodeId)) continue;

			const road = new Road(pos, this.trafficMap.nodes.get(nodeId).pos, id, nodeId, edgePath[nodeId]);
			this.roads.set(`${id}-${nodeId}`, road)
			intersection.roads.push(`${id}-${nodeId}`)
			this.intersections.get(nodeId)?.roads.push(`${id}-${nodeId}`)
		}
		this.intersections.set(id, intersection)
	}

	bestRoute(node1Id, node2Id, currRoadId) {
		const { cost, road } = this.cost(currRoadId, node1Id, node2Id);
		// console.log(`cost: ${Math.round(cost)}, via ${road.node1Id}-${road.node2Id}`)
		return { cost, road }
	}

	shortestDist(node1Id, node2Id, currRoadId) {
		const tmpAlgorithm = algorithm
		algorithm = 's'
		const { cost, _ } = this.cost(currRoadId, node1Id, node2Id)
		algorithm = tmpAlgorithm
		return cost
	}


	cost(currRoadId, node1Id, finalNodeId, visitedRoads = []) {
		if (!this.intersections.has(node1Id) || !this.intersections.has(finalNodeId) || (!this.roads.has(currRoadId) && currRoadId)) return;
		if (node1Id == finalNodeId) return { cost: 0, road: null }

		const node1 = this.intersections.get(node1Id)
		if (node1.costs.has(finalNodeId) && node1.costs.get(finalNodeId).cost != Infinity && node1.costs.get(finalNodeId).lastUpdate > new Date().valueOf() - updateInterval) {
			return node1.costs.get(finalNodeId)
		}

		let lowestCost = Infinity;
		let fastestDirection = null;
		for (const roadId of node1.roads) {
			if (roadId == currRoadId || visitedRoads.includes(roadId)) continue;
			const road = this.roads.get(roadId)
			const otherEndId = road.node1Id == node1Id ? road.node2Id : road.node1Id;
			const cost = road.cost(node1Id, algorithm) + this.cost(`${road.node1Id}-${road.node2Id}`, otherEndId, finalNodeId, [roadId, ...visitedRoads]).cost
			if (cost < lowestCost) {
				lowestCost = cost;
				fastestDirection = road;
			}
		}
		// node1.costs.set(finalNodeId, { cost: lowestCost, road: fastestDirection, lastUpdate: new Date().valueOf() })
		// if (lowestCost < Infinity) console.log(`${lowestCost}  via ${fastestDirection?.node1Id}-${fastestDirection?.node2Id} from ${node1Id} to ${finalNodeId}`)
		return { cost: lowestCost, road: fastestDirection }
	}

	update(sim) {
		for (const [_id, road] of this.roads) {
			road.update(sim)
		}
	}

	render() {
		let shortestDist = Infinity;
		let nearestPt = createVector(0, 0)
		this.roads.forEach((road, id) => {
			strokeWeight(10)
			stroke(150)
			noFill()
			beginShape()
			vertex(road.p1.x, road.p1.y)
			road.vertices.forEach(v => vertex(v.x, v.y))
			vertex(road.p2.x, road.p2.y)
			endShape()

			// const theta = Math.atan2((road.p2.y - road.p1.y), (road.p2.x - road.p1.x))
			// stroke(255, 220, 0)
			// line(road.p1.x, road.p1.y, road.p1.x + road.pathLength * Math.cos(theta), road.p1.y + road.pathLength * Math.sin(theta))

			textSize(15)
			fill(255)
			noStroke()
			text(id, road.p1.x / 2 + road.p2.x / 2, road.p1.y / 2 + road.p2.y / 2)

			const [d, p] = road.distanceToPoint(createVector(mouseX, mouseY))
			if (d < shortestDist) {
				shortestDist = d;
				nearestPt = p
			}
		})
		this.intersections.forEach(intersection => {
			strokeWeight(13)
			stroke(255)
			point(intersection.pos.x, intersection.pos.y)
			textSize(15)
			fill(255)
			noStroke()
			text(intersection.id, intersection.pos.x + 15, intersection.pos.y + 15)
		})
		stroke(50, 250, 255, 200)
		strokeWeight(15)
		stroke(0, 255, 0, 200)
	}

}