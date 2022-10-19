

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

const mapData = map1