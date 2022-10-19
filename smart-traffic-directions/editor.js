

class MapEditor {
    /**
     * 
     * @param {RoadMap} map 
     */
    constructor(map) {
        this.map = map;
        // this.edgeColor = [200, 200, 200]
        this.edgeColor = [20, 20, 20]
        // this.nodeColor = [255, 255, 255]
        this.nodeColor = [100, 100, 100]
        this.edgeSize = 10;
        this.selectionItem = null;
        this.selectionType = null;
        this.hoverItem = null;
        this.hoverType = null;
        this.tmpPath = [];
        this.tmpStartNode = null;
        this.tmpEndNode = null;
        this.currKey = null;
    }

    onKeyPress() {
        this.currKey = key


        if (this.currKey == 'b') {
            if (this.hoverType == 'node') {
                this.tmpStartNode = this.hoverItem
            } else {
                const id = Math.round(100 * Math.random)
                this.map.createIntersection(id, createVector(mouseX, mouseY))
                this.tmpStartNode = this.map.intersections.get(id)
            }
            this.tmpPath.push(this.tmpStartNode.pos)
        }

        if (this.currKey == 'e' && this.tmpStartNode != null) {
            if (this.hoverType == 'node') {
                this.tmpEndNode = this.hoverItem
            } else {
                const id = Math.round(100 * Math.random())
                this.map.createIntersection(id, createVector(mouseX, mouseY))
                this.tmpEndNode = this.map.intersections.get(id)
            }
            this.tmpPath.push(this.tmpEndNode.pos)
            const roadId = `${this.tmpStartNode.id}-${this.tmpEndNode.id}`
            this.map.roads.set(roadId, new Road(this.tmpPath[0], this.tmpPath[this.tmpPath.length - 1], this.tmpStartNode.id, this.tmpEndNode.id, this.tmpPath.slice(1, this.tmpPath.length - 1)))
            this.tmpStartNode.connections.push(this.tmpEndNode.id)
            this.tmpEndNode.connections.push(this.tmpStartNode.id)
            this.tmpStartNode.roads.push(roadId)
            this.tmpEndNode.roads.push(roadId)
            this.tmpPath = [];
            this.tmpStartNode = null;
            this.tmpEndNode = null;
        }
    }

    onKeyRelease() {
        this.currKey = null
    }

    createMode() {

    }

    onMouseClick() {
        this.selectionItem = this.hoverItem
        this.selectionType = this.hoverType

        if (this.tmpPath.length > 0) {
            this.tmpPath.push(createVector(mouseX, mouseY))
        }

    }

    show() {
        this.hoverItem = null
        this.hoverType = null
        const mouse = createVector(mouseX, mouseY)
        for (const [_, edge] of this.map.roads) {
            let prevV = edge.vertices[0]
            stroke(this.edgeColor[0], this.edgeColor[1], this.edgeColor[2])
            strokeWeight(this.edgeSize)
            noFill()
            beginShape()
            for (const currV of edge.vertices) {
                vertex(currV.x, currV.y)

                const a = p5.Vector.sub(mouse, prevV)
                const b = p5.Vector.sub(currV, prevV)
                const bMag = b.mag()
                const v = constrain(a.dot(b.normalize()), 0, bMag)
                const r = p5.Vector.add(prevV, b.mult(v))
                if (dist(r.x, r.y, mouse.x, mouse.y) <= this.edgeSize + 4) {
                    //     strokeWeight(this.edgeSize * 1.25);
                    //     stroke(0, 255, 200)
                    //     point(r.x, r.y)
                    this.hoverItem = edge
                    this.hoverType = 'edge'
                }
                prevV = currV
            }
            endShape()
            textSize(15)
            fill(255)
            noStroke()
            text(`${(edge.costFwd + edge.dCostFwd).toFixed(2)}, ${(edge.costBck + edge.dCostBck).toFixed(2)}`, edge.p1.x / 2 + edge.p2.x / 2, edge.p1.y / 2 + edge.p2.y / 2)

        }

        for (const [_, intersection] of this.map.intersections) {
            stroke(this.nodeColor[0], this.nodeColor[1], this.nodeColor[2])
            strokeWeight(this.edgeSize * 1.25)
            point(intersection.pos.x, intersection.pos.y)

            if (dist(intersection.pos.x, intersection.pos.y, mouse.x, mouse.y) <= this.edgeSize * 1.25 + 4) {
                this.hoverItem = intersection
                this.hoverType = 'node'
            }

            textSize(15)
            fill(255)
            noStroke()
            text(intersection.id, intersection.pos.x + 15, intersection.pos.y + 15)
        }

        if (this.selectionType == 'node') {
            stroke(50, 255, 200)
            strokeWeight(2)
            noFill()
            circle(this.selectionItem.pos.x, this.selectionItem.pos.y, this.edgeSize * 2)
        } else if (this.selectionType == 'edge') {
            stroke(255, 100, 240)
            strokeWeight(this.edgeSize * 1.1)
            noFill()
            beginShape()
            this.selectionItem.vertices.forEach(v => vertex(v.x, v.y))
            endShape()
        }

        if (this.hoverType == 'node') {
            stroke(50, 255, 200)
            strokeWeight(this.edgeSize * 1.5)
            point(this.hoverItem.pos.x, this.hoverItem.pos.y)
        } else if (this.hoverType == 'edge') {
            stroke(255, 50, 200)
            strokeWeight(this.edgeSize * 1.25)
            noFill()
            beginShape()
            this.hoverItem.vertices.forEach(v => vertex(v.x, v.y))
            endShape()
        }

        if (this.tmpPath.length > 0) {
            beginShape()
            stroke(200, 50, 200)
            noFill()
            strokeWeight(this.edgeSize * 1.28)
            for (const p of this.tmpPath) {
                vertex(p.x, p.y)
            }
            vertex(mouseX, mouseY)
            endShape()
        }


        textSize(15)
        fill(255)
        noStroke()
        text(`t_av: ${sim.avgTravelTime}`, width - 100, 20)
    }
}