
class Vehicle {
    /**
     * 
     * @param {RoadMap} roadMap 
     */
    constructor(id, roadMap, sim, startNode, targetNode) {
        this.id = id
        this.pos = 0
        this.roadMap = roadMap
        this.posHistory = [0]
        this.currPathSegment = []
        this.currPathSegmentIdx = null
        this.vehicleInfront = null
        this.vehicleBehind = null
        this.currRoad = null
        this.color = Math.random() * 360
        this.startNodeId = startNode
        this.targetNodeId = targetNode
        this.creationTime = new Date().valueOf()
        this.travelledDist = 0
        this.frameCount = 0;

        this.maxVel = 50 + Math.random() * 50
        this.maxAcc = 15 + Math.random() * 15
        this.dir = 0
        this.vel = 0
        this.idealClearance = 5 + Math.random() * 5;
        this.lookAheadDist = 50 + Math.random() * 100;

        this.currRoad = this.roadMap.bestRoute(startNode, this.targetNodeId).road
        if (this.currRoad) {
            this.enterRoad(this.currRoad.id(), this.startNodeId, sim, roadMap)
        }
    }

    setInfront(infrontId, sim) {
        this.vehicleInfront = sim.vehicles.get(infrontId)
    }

    getPos(deltaFrame = 0) {
        return this.posHistory[constrain(this.posHistory.length - Math.round(deltaFrame) - 1, 0, this.posHistory.length - 1)]
    }

    move(scaling = 1) {
        let desiredVel = this.maxVel
        if (this.currRoad.onSameLane(this.id, this.vehicleInfront?.id) &&
            //some edge cases apparently====================
            this.id != this.vehicleInfront?.id &&
            this.id != this.vehicleInfront?.vehicleInfront?.id
            //==============================================
        ) {
            if (this.dir * (this.vehicleInfront.pos - this.pos) <= 0) {
                // const tmpInfront = this.sim.vehicles.get(this.vehicleInfront.vehicleInfront?.id)
                // this.vehicleInfront.vehicleInfront = this
                // // this.vehicleInfront.vehicleBehind = this.sim.vehicles.get(this.vehicleBehind?.id)
                // // this.vehicleBehind = this.sim.vehicles.get(this.vehicleInfront.id)
                // this.vehicleInfront = tmpInfront
                // this.move(scaling)
                if (Math.random() < 0.8) this.pos = this.vehicleInfront.pos
                return
            }

            const distanceToInfront = this.dir * (this.vehicleInfront.getPos(30 / (deltaTime / 16)) - this.getPos(30 / (deltaTime / 16)))
            desiredVel = this.maxVel * (distanceToInfront - this.idealClearance) / (this.idealClearance)
        }
        desiredVel = constrain(desiredVel, 0, this.maxVel)

        const dv = desiredVel - this.vel
        this.acc = Math.sign(dv) * this.maxAcc
        if (this.dir * dv < 0) {
            this.acc = Math.sign(dv) * this.maxAcc * 4
        }
        this.vel += scaling * this.acc * deltaTime / 1000
        // this.vel = desiredVel

        if (Math.random() < 0.01) this.vel /= (2.5 + Math.random() * 2)
        if (Math.random() <= 0.0002 && this.pos / this.currRoad.pathLength > 0.2 && this.pos / this.currRoad.pathLength < 0.8) {
            const actualMaxVel = this.maxVel
            this.maxVel = 0
            setTimeout(() => this.maxVel = 50 + Math.random() * 150, 5000 / scaling)
        }

        this.posHistory.push(this.pos)
        if (this.posHistory.length > 90) {
            this.posHistory.shift()
        }
        this.pos += scaling * this.dir * this.vel * deltaTime / 1000
        // this.pos += scaling * this.dir * 0.1 * constrain(this.maxVel * velFactor, 0, this.maxVel) /* (this.vehicleInfront?.maxVel === 0 ? 0 : 1)*/

    }


    enterRoad(roadId, nodeId, sim, roadMap) {
        this.vel = this.maxVel
        this.currRoad = roadMap.roads.get(roadId)
        const { dir, pathSegment, infront, pos } = this.currRoad.enter(this.id, nodeId, sim)
        this.vehicleInfront = sim.vehicles.get(infront)
        this.dir = dir
        this.pos = pos
        this.posHistory = [pos]
        this.currPathSegment = this.currRoad.vertices.slice(pathSegment, pathSegment + 2)
        this.currPathSegmentIdx = pathSegment
    }

    update(speed = 1, sim, roadMap) {
        if (!this.currRoad) return false
        this.frameCount += 1
        this.move(speed)

        const { exitNode, newPathSegment } = this.currRoad.updatePos(this.id, this.pos, this.currPathSegmentIdx,)
        this.currPathSegmentIdx = newPathSegment
        this.currPathSegment = this.currRoad.vertices.slice(newPathSegment, newPathSegment + 2)
        if (exitNode) {
            if (exitNode == this.targetNodeId) {
                return 'f'
            }
            this.vehicleInfront = null
            this.travelledDist += this.currRoad.pathLength
            // const newRoad = this.pickRoad(this.currRoad, this.roadMap.intersections.get(exitNode))
            const newRoad = this.roadMap.bestRoute(exitNode, this.targetNodeId, this.currRoad.id()).road
            if (newRoad) {
                this.enterRoad(newRoad.id(), exitNode, sim, roadMap)
            } else {
                this.currRoad = null
                return false
            }
        }
        if (newPathSegment === undefined) {
            this.currRoad = null
        }

        return true

    }

    show(editor) {
        if (!this.currRoad || this.pos < 0 || (this.currRoad && this.pos > this.currRoad.pathLength)) { this.currRoad = null; return }
        const relPos = this.pos / this.currRoad.pathLength
        const p1 = this.currPathSegment[0]
        const p2 = this.currPathSegment[1]


        const [p1Rel, p2Rel] = this.currRoad.pathSegments[this.currPathSegmentIdx]
        const relRelPos = (relPos - p1Rel) / (p2Rel - p1Rel)
        const slope = (p2.y - p1.y) / (p2.x - p1.x)
        // const slopePerpendicular = -1 / slope
        const dx = p2.x - p1.x
        const theta = Math.atan2(p2.y - p1.y, p2.x - p1.x) + Math.PI / 2

        const posX = p1.x + relRelPos * dx + this.dir * 4 * cos(theta)
        const posY = p1.y + slope * relRelPos * dx + this.dir * 4 * sin(theta)



        colorMode(HSB)
        stroke(0)
        fill(this.color, 50, 100)
        strokeWeight(1)
        colorMode(RGB)
        // if (this.dir > 0) fill(0, 50, 100)
        // else fill(100, 50, 0)
        circle(posX, posY, 10)
        textSize(8)
        fill(0)
        stroke(255)
        textAlign(CENTER, CENTER)
        if (editor.currKey == 'd') text(`${this.id}`, posX + 10, posY)
        // if (this.vehicleInfront) {
        //     fill(255)
        //     stroke(0)

        //     circle(posX, posY, 5)
        //     line(posX, posY, this.vehicleInfront.posX, this.vehicleInfront.posY)
        // }
    }

    pickRoad(currRoad, intersection) {
        let availableRoads = intersection.roads.filter(rId => rId != currRoad.id())
        // if (availableRoads.length <= 0) availableRoads = intersection.roads
        const newRoadId = availableRoads[Math.round(Math.random() * (availableRoads.length - 1))]
        if (newRoadId)
            return this.roadMap.roads.get(newRoadId)
    }
}