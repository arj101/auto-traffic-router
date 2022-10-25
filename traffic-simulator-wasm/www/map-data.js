

const map1 = [

	{
		id: '1',
		pos: [150, 200],
	},
	{
		id: '2',
		pos: [250, 200],
	},
	{
		id: 'a',
		pos: [100, 100],
		connections: ['1', '2'],
	},
	{
		id: 'b',
		pos: [300, 300],
		connections: ['1', '2']
	},
	{
		id: 'c',
		pos: [250, 100],
		connections: ['a', '2']
	}

]

const map2 = [
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
		connections: ['1', '4']
	},
	{
		id: 'b',
		pos: [311, 440],
		connections: ['2', '3']
	},
	{
		id: 'c',
		pos: [609, 366],
		connections: ['6', '3']
	},
	{
		id: 'd',
		pos: [579, 167],
		connections: ['5', '6']
	}
]

const map3 = {
	intersections: [
		{
			id: 'a',
			pos: [100, 100]
		},
		{
			id: '1',
			pos: [150, 150]
		},
		{
			id: '2',
			pos: [100, 200]
		},
		{
			id: '3',
			pos: [250, 200],
		},
		{
			id: '4',
			pos: [300, 300]
		},
		{
			id: 'b',
			pos: [400, 400]
		},
	],
	roads: [
		{
			n1: 'a',
			n2: '1'
		},
		{
			n1: '1',
			n2: '2'
		},
		{
			n1: '1',
			n2: '3'
		},
		{
			n1: '3',
			n2: '4'
		},
		{
			n1: '2',
			n2: '4'
		},
		{
			n1: '4',
			n2: 'b'
		},
		{
			n1: 'b',
			n2: '3'
		},
		{
			n1: 'a',
			n2: '2'
		}
	]
}

const map4 = {
	intersections: [
		['a', 100, 100, 100],
		['1', 150, 150, 10],
		['2', 100, 200, 10],
		['3', 200, 200, 25],
		['4', 200, 250, 10],
		['b', 300, 300, 100],
	],
	roads: [
		['a', '1'],
		['a', '2'],
		['1', '2'],
		['2', '4'],
		['1', '3'],
		['3', 'b'],
		['4', 'b'],
		['3', '4']

	]
}

const mapData = map4
module.exports = mapData;