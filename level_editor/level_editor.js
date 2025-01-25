let canvas = document.getElementById("canvas");
let ctx = canvas.getContext("2d");

let camera_x = 0;
let camera_y = 0;

async function fetchData() {
    let response = await fetch('../src/game_data/starting_level.json');
    let json = await response.json();
    console.log(json);
}

fetchData();


class Terrain {
    constructor(x, y, width, height, archetype) {
        this.x = x;
        this.y = y;
        this.width = width;
        this.height = height;
        this.archetype = archetype;
    }
}

function draw() {
    ctx.clearRect(0, 0, canvas.width, canvas.height);
    ctx.fillStyle = "black";
    ctx.fillRect(0, 0, canvas.width, canvas.height);
}

setInterval(draw, 1000 / 60);