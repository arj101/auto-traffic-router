

class Simulator {
    /**
     * 
     * @param {RoadMap} roadMap 
     */
    constructor(roadMap) {
        this.roadMap = roadMap
        this.vehicles = new Map()
        this.state = true
        this.speed = 0.3
        this.spawnProbability = 0.3
        this.maxSpawnsPerFrame = 5
        this.count = 0
        this.travelTimeSum = 0
        this.travelTimeN = 0
        this.avgTravelTime = 0
        this.intersectionIds = []
        this.majorIntersectionIds = []
        this.averages = new Map();

        for (const intersection of this.roadMap.intersections.keys()) {
            if (/^[a-z]+$/.test(intersection)) this.majorIntersectionIds.push(intersection)
            else this.intersectionIds.push(intersection)
        }


        this.spawnVehicles(10, 'a', 'c')


    }

    clearAvg() {
        this.travelTimeN = 0
        this.travelTimeSum = 0
    }

    pickNode() {
        if (Math.random() < 0.65) {
            return this.roadMap.intersections.get(this.majorIntersectionIds[Math.round(Math.random() * (this.majorIntersectionIds.length - 1))])
        }
        return this.roadMap.intersections.get(this.intersectionIds[Math.round(Math.random() * (this.intersectionIds.length - 1))])
    }

    spawnVehicles(n = 5, s, t) {
        for (let i = 0; i < n; i++) {
            const startNode = this.pickNode()
            let endNode = this.pickNode()
            while (endNode.id == startNode.id) {
                endNode = this.pickNode()

            }
            // const v = new Vehicle(this.count, this.roadMap, this, startNode.id, endNode.id)
            const v = new Vehicle(this.count, this.roadMap, this, s || startNode.id, t || endNode.id)
            // v.enterRoad(road.id(), Math.random() > 0.5 ? road.node1Id : road.node2Id)
            // console.log(`${startNode.id} to ${endNode.id}`)
            this.vehicles.set(this.count, v)
            this.count += 1
        }
    }

    run(editor, map) {
        if (!this.state) return
        if (Math.random() < this.spawnProbability && this.vehicles.size < 500) this.spawnVehicles(this.maxSpawnsPerFrame * Math.random())
        // this.spawnProbability *= 0.1 + Math.random() * 2
        const currT = new Date().valueOf()
        for (const [idx, vehicle] of this.vehicles) {
            const v = vehicle.update(this.speed, this, map)
            if (v !== true) {
                if (v == 'f') {
                    if (!this.majorIntersectionIds.includes(vehicle.startNodeId) || !this.majorIntersectionIds.includes(vehicle.targetNodeId)) continue
                    let prevSum = this.averages.get(`${vehicle.startNodeId} ${vehicle.targetNodeId}`)?.sum || 0
                    let prevN = this.averages.get(`${vehicle.startNodeId} ${vehicle.targetNodeId}`)?.n || 0
                    prevSum += (currT - vehicle.creationTime) / 1000
                    prevN += 1
                    this.avgTravelTime = this.travelTimeSum / this.travelTimeN
                    this.averages.set(`${vehicle.startNodeId} ${vehicle.targetNodeId}`, { sum: prevSum, n: prevN })
                }
                this.vehicles.delete(idx)
                continue
            }
            vehicle.show(editor)
        }
        // if(Math.random() < 0.05) {
        // for (const [idx, v] of this.vehicles) {
        //     // if (Math.random() < 0.01) v.maxVel = 0
        // }
        let off_y = 0;
        textSize(15)
        fill(255)
        noStroke()
        for (const [route, { sum, n }] of this.averages) {
            text(`${route}: ${sum / n}`, width - 80, 20 + off_y)
            off_y += 20;
        }
    }

}
