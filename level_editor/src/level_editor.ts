import P5 from 'p5';

class Terrain {
	x: number;
	y: number;
	width: number;
	height: number;
	terrain_archetype: string;
    constructor(x: number, y: number, width: number, height: number, terrain_archetype: string) {
        this.x = x;
        this.y = y;
        this.width = width;
		this.height = height;
        this.terrain_archetype = terrain_archetype;
    }
}

class Entity {
	x: number;
	y: number;
	archetype: string;
	sprite: string;
    constructor(x: number, y: number, archetype: string, sprite:string) {
        this.x = x;
        this.y = y;
        this.archetype = archetype;
        this.sprite = sprite;
    }
}

class TerrainArchetype {
	name: string;
	type: string;
	random_chances: number[];
	sprites: string[];
	basic_tags: string[];
    constructor(name: string, type: string, random_chances: number[], sprites: string[], basic_tags: string[]) {
        this.name = name;
        this.type = type;
        this.random_chances = random_chances;
        this.sprites = sprites;
        this.basic_tags = basic_tags;
    }
}

interface CollisionBox {
	x_offset: number,
	y_offset: number,
	width: number,
	height: number
}

class EntityArchetype {
	name: string;
	basic_tags: string[];
	collision_box: CollisionBox;
	damage_box: CollisionBox;
	health: number;
	monster_type: string;
	movement_speed: number;
	range: number;
	aggro_range: number;
	attack_type: string;
	attack_pattern: string;
	loot_table: string[];
    constructor(name: string, basic_tags: string[], collision_box: CollisionBox, damage_box: CollisionBox, health: number, monster_type: string, movement_speed: number, range: number, aggro_range: number, attack_type: string, attack_pattern: string, loot_table: string[]) {
        this.name = name;
        this.basic_tags = basic_tags;
        this.collision_box = collision_box;
        this.damage_box = damage_box;
        this.health = health;
        this.monster_type = monster_type;
        this.movement_speed = movement_speed;
        this.range = range;
        this.aggro_range = aggro_range;
        this.attack_type = attack_type;
        this.attack_pattern = attack_pattern;
        this.loot_table = loot_table;
    }
}


class Sprite {
	image_id: number;
	width: number;
	height: number;
	x: number;
	y: number;
	total_width: number;
	total_height: number;
    constructor(image_id: number, width: number, height: number, x: number, y: number, total_width: number, total_height: number) {
        this.image_id = image_id;
        this.width = width;
        this.height = height;
        this.x = x;
        this.y = y;
        this.total_width = total_width;
        this.total_height = total_height;
    }
}



class GeneralQuery {
	x: number;
	y: number;
	terrain: number[];
	entities: number[];
	constructor(x: number, y: number, terrain: number[], entities: number[]) {
		this.x = x;
		this.y = y;
		this.terrain = terrain;
		this.entities = entities;
	}
}


class EntityQuery {}
class TerrainQuery {
	terrain_id: number;
	constructor(terrain_id: number) {
		this.terrain_id = terrain_id;
	}
}
let camera_x = 0;
let camera_y = 0;

let terrain: Terrain[] = [];
let entities: Entity[] = [];
let terrain_archetypes: TerrainArchetype[] = [];
let entity_archetypes: EntityArchetype[] = [];
let sprites: Map<string, Sprite> = new Map();
let paths: string[] = [];
let images: P5.Image[] = [];
let json1 = null;


async function fetchData() {
    let response1 = await fetch('../src/game_data/starting_level.json');
    json1 = await response1.json();
    let response2 = await fetch('../src/game_data/entity_archetypes.json');
    let json2 = await response2.json();
    let response3 = await fetch('../src/game_data/terrain_archetypes.json');
    let json3 = await response3.json();
    for (let i = 0; i < json1.terrain.length; i++) {
        let t = json1.terrain[i];
        terrain.push(new Terrain(t.x, t.y, t.width, t.height, t.terrain_archetype));
    }
    for (let i = 0; i < json1.entities.length; i++) {
        let e = json1.entities[i];
        entities.push(new Entity(e.x, e.y, e.archetype, e.sprite));
    }
    for (let i = 0; i < json3.length; i++) {
        let ta = json3[i];
        terrain_archetypes.push(new TerrainArchetype(ta.name, ta.type, ta.random_chances, ta.sprites, ta.basic_tags));
    }
    for (let i = 0; i < json2.length; i++) {
        let ea = json2[i];
        entity_archetypes.push(new EntityArchetype(ea.name, ea.basic_tags, ea.collision_box, ea.damage_box, ea.health, ea.monster_type, ea.movement_speed, ea.range, ea.aggro_range, ea.attack_type, ea.attack_pattern, ea.loot_table));
    }
    let response4 = await fetch('../src/game_data/sprites.json');
    let json4 = await response4.json();
    for (let i = 0; i < json4.basic_sprites.length; i++) {
        let s = json4.basic_sprites[i];
        let ss = new Sprite(paths.length, 32, 32, 0, 0, 32, 32);
        sprites.set(s.name, ss);
        paths.push("../" + s.path);
    }
    for (let i = 0; i < json4.spritesheets.length; i++) {
        let s = json4.spritesheets[i];
        for (let j = 0; j < s.sprites.length; j++) {
            let ss = s.sprites[j];
            let sss = new Sprite(paths.length, s.sprite_width, s.sprite_height, ss.x * (s.sprite_width + s.sprite_padding), ss.y * (s.sprite_height + s.sprite_padding), s.width, s.height);
            sprites.set(ss.name, sss);
        }
        paths.push("../" + s.path);
    }

}

let cur_query: GeneralQuery | EntityQuery | TerrainQuery = new GeneralQuery(0, 0, [], []);
function qb_clicked(terrain: boolean, id: number) {

	if (terrain && cur_query instanceof GeneralQuery) {
		display_terrain(cur_query.terrain[id]);
	}
}
document.getElementById("save").addEventListener("click", save);

function display_terrain(id: number) {
	let ter = terrain[id];
	let ta = terrain_archetypes.find(x => x.name === ter.terrain_archetype);
	if (ta == undefined) return;
	let ihtml = "";
	ihtml += "<label> x: </label> <input class = 'query_input' type='number' id='ctx' value='" + ter.x + "'><br><br>";
	ihtml += "<label> y: </label> <input class = 'query_input' type='number' id='cty' value='" + ter.y + "'><br><br>";
	ihtml += "<label> w: </label> <input class = 'query_input' type='number' id='ctw' value='" + ter.width + "'><br><br>";
	ihtml += "<label> h: </label> <input class = 'query_input' type='number' id='cth' value='" + ter.height + "'><br><br>";
	let select = "<select class = 'query_input' id = 'cta'>";
	for (let i = 0; i < terrain_archetypes.length; i++) {
		if (terrain_archetypes[i].name === ter.terrain_archetype) {
			select += "<option selected>" + terrain_archetypes[i].name + "</option>";
		}else{
			select += "<option>" + terrain_archetypes[i].name + "</option>";
		}
	}
	select += "</select>";
	ihtml += "<label> Archetype: </label>" + select + "<br><br>";
	ihtml += "<p> Type: " + ta.type + "</p>";
	if (ta.type == "randomness") {
		ihtml += "<p> Random Chances: ";
		for (let i = 0; i < ta.random_chances.length; i++) {
			if (i == ta.random_chances.length - 1) {
				ihtml += ta.random_chances[i];
				break;
			}
			ihtml += ta.random_chances[i] + ", ";
		}
		ihtml += "</p>";
	}
	ihtml += "<p> Sprites: ";
	for (let i = 0; i < ta.sprites.length; i++) {
		if (i == ta.sprites.length - 1) {
			ihtml += ta.sprites[i];
			break;
		}
		ihtml += ta.sprites[i] + ", ";
	}
	ihtml += "</p>";
	ihtml += "<p> Basic Tags: ";
	for (let i = 0; i < ta.basic_tags.length; i++) {
		if (i == ta.basic_tags.length - 1) {
			ihtml += ta.basic_tags[i];
			break;
		}
		ihtml += ta.basic_tags[i] + ", ";
	}
	ihtml += "</p>";
	cur_query = new TerrainQuery(id);
	document.getElementById("query_text").innerHTML = "Terrain";
	document.getElementById("query_data").innerHTML = ihtml;
	document.getElementById("ctx").addEventListener("change", function() { ter.x = parseInt((<HTMLInputElement>document.getElementById("ctx")).value); });
	document.getElementById("cty").addEventListener("change", function() { ter.y = parseInt((<HTMLInputElement>document.getElementById("cty")).value); });
	document.getElementById("ctw").addEventListener("change", function() { ter.width = parseInt((<HTMLInputElement>document.getElementById("ctw")).value); });
	document.getElementById("cth").addEventListener("change", function() { ter.height = parseInt((<HTMLInputElement>document.getElementById("cth")).value); });
	document.getElementById("cta").addEventListener("change", function() { ter.terrain_archetype = (<HTMLSelectElement>document.getElementById("cta")).value; });
}

function save() {
	json1.terrain = terrain;
	json1.entities = entities;
	const blob = new Blob([JSON.stringify(json1)], { type: 'application/json' });
	const link = document.createElement('a');
	link.href = window.URL.createObjectURL(blob);
	link.download = 'starting_level.json';
	link.click();

	alert("GAME DATA DOWNLOAD SHOULD BEGIN, MOVE DATA INTO GAME DATA FOLDER, rename the file to starting_level.json if it isn't");
}

function display_entity(id: number) {
}

const sketch = (p5: P5) => { 
	p5.preload = async function() {
		await fetchData();
		for (let i = 0; i < paths.length; i++) {
			images.push(p5.loadImage(paths[i]));
		}
	}
	p5.setup = function () {
		p5.createCanvas(1152, 720);
	}

	let randomness = Array(100).fill(undefined).map(()=>Array(100).fill(undefined));

	p5.draw = function draw() {
		p5.background(220);
		for (let terrain_tile in terrain) {
			let t = terrain[terrain_tile];
			let ta = terrain_archetypes.find(x => x.name === t.terrain_archetype);
			if (ta == undefined){
				continue;
			}
			let s = sprites.get(ta.sprites[0]);
			if (ta.type === "randomness") {
				for (let i = 0; i < t.width; i++) {
					for (let j = 0; j < t.height; j++) {
						if (randomness[t.x + i][t.y + j] === undefined) {
							let random = Math.random();
							let sum = 0;
							for (let w = 0; w < ta.random_chances.length; w++) {
								sum += ta.random_chances[w];
								if (random < sum) {
									randomness[t.x + i][t.y + j] = w;
									s = sprites.get(ta.sprites[w]);
									break;
								}
							}
						}else{
							s = sprites.get(ta.sprites[randomness[t.x + i][t.y + j]]);
						}
						if (s == undefined) continue;
						if (t.x * 32 + i * 32.0 - camera_x <= -32 || t.x * 32 + i * 32.0 + 32.0 - camera_x > p5.width + 32 || t.y * 32 + j * 32.0 - camera_y < -32 || t.y * 32 + j * 32.0 + 32.0 - camera_y > p5.height + 32) {
							continue;
						}
						p5.imageMode(p5.CORNER);
						p5.noStroke();
						p5.noSmooth();
						p5.image(images[s.image_id], t.x * 32 + i * 32.0 - camera_x, t.y * 32 + j * 32.0 - camera_y, 32, 32, s.x, s.y, s.width, s.height);
					}
				}
			}else{
				if (s == undefined) continue;
				for (let i = 0; i < t.width; i++) {
					for (let j = 0; j < t.height; j++) {
						if (t.x * 32 + i * 32.0 - camera_x <= -32 || t.x * 32 + i * 32.0 + 32.0 - camera_x > p5.width + 32 || t.y * 32 + j * 32.0 - camera_y < -32 || t.y * 32 + j * 32.0 + 32.0 - camera_y > p5.height + 32) {
							continue;
						}
						p5.imageMode(p5.CORNER);
						p5.noStroke();
						p5.noSmooth();
						p5.image(images[s.image_id], t.x * 32 + i * 32.0 - camera_x, t.y * 32 + j * 32.0 - camera_y, 32, 32, s.x, s.y, s.width, s.height);
					}
				}
			}
		}
		for (let entity in entities) {
			let e = entities[entity];
			let ea = entity_archetypes.find(x => x.name === e.archetype);
			if (ea == undefined){
				continue;
			}
			let s = sprites.get(e.sprite);
			if (s == undefined) continue;
			if (e.x - camera_x <= -32 || e.x + 32 - camera_x > p5.width + 32 || e.y - camera_y < -32 || e.y + 32 - camera_y > p5.height + 32) {
				continue;
			}
			p5.imageMode(p5.CORNER);
			p5.noStroke();
			p5.noSmooth();
			p5.image(images[s.image_id], e.x - camera_x, e.y - camera_y, 32, 32, s.x, s.y, s.width, s.height);
		}
		for (let x = Math.floor(camera_x/32) * 32; x < p5.width + camera_x; x += 32) {
			for (let y = Math.floor(camera_y/32) * 32; y < p5.height + camera_y; y += 32) {
				p5.stroke(110);
				p5.strokeWeight(0.5);
				p5.noFill();
				p5.rect(x - camera_x, y - camera_y, 32, 32);
			}
		}
		let mouse_x = p5.mouseX + camera_x;
		let mouse_y = p5.mouseY + camera_y;
		if (p5.mouseX > p5.width - 5 || p5.mouseY > p5.height - 5){}else{
			p5.noStroke();
			p5.fill(0, 0, 0, 100);
			let mouse_rect_x = Math.floor(mouse_x / 32) * 32;
			let mouse_rect_y = Math.floor(mouse_y / 32) * 32;
			p5.rect(mouse_rect_x - camera_x, mouse_rect_y - camera_y, 32, 32);
		}
	
		if (p5.keyIsDown(p5.LEFT_ARROW) || p5.keyIsDown(65)) {
			camera_x -= 5;
		}
		if (p5.keyIsDown(p5.RIGHT_ARROW) || p5.keyIsDown(68)) {
			camera_x += 5;
		}
		if (p5.keyIsDown(p5.UP_ARROW) || p5.keyIsDown(87)) {
			camera_y -= 5;
		}
		if (p5.keyIsDown(p5.DOWN_ARROW) || p5.keyIsDown(83)) {
			camera_y += 5;
		}
	}
	

	p5.mousePressed = function mousePressed() {
		let mouse_x = p5.mouseX + camera_x;
		let mouse_y = p5.mouseY + camera_y;
		if (p5.mouseX > p5.width - 5 || p5.mouseY > p5.height - 5) return;
		let mouse_rect_x = Math.floor(mouse_x / 32) * 32;
		let mouse_rect_y = Math.floor(mouse_y / 32) * 32;
		document.getElementById("query_text").innerHTML = "Query at x: " + mouse_rect_x + " y: " + mouse_rect_y;

		let ihtml = "";
		let terrain_queried = [];
		for (let i = 0; i < terrain.length; i++) {
			if (terrain[i].x <= mouse_rect_x/32 && terrain[i].x + terrain[i].width > mouse_rect_x/32 && terrain[i].y <= mouse_rect_y/32 && terrain[i].y + terrain[i].height > mouse_rect_y/32) {
				ihtml += "<button id = 'cqueryt" + i + "'> "+ terrain[i].terrain_archetype + "</button>";
				terrain_queried.push(i);
			}
		}

		cur_query = new GeneralQuery(mouse_rect_x, mouse_rect_y, terrain_queried, []);
	

		document.getElementById("query_data").innerHTML = ihtml;
		for (let i = 0; i < terrain_queried.length; i++) {
			document.getElementById("cqueryt" + terrain_queried[i]).addEventListener("click", function() {
				qb_clicked(true,i);
			});
		}
	}
}

new P5(sketch);
