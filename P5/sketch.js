let fluid;

function printField(field) {
  let s = "";
  for (let i = 0; i < field.length; i++) {
      s += field[i] + ", ";
      if ((i % N) == N - 1) {
          console.log(Math.floor(i / N) + " " + s);
          s = "";
      }
  }
}

function setup() {
  let width = 300;
  let height = 300;
  createCanvas(width, height);
  frameRate(60);
  fluid = new Fluid(0.2, 1.0, 0.0000001);
  fluid.addDensity(2, 2, 1.0);
  fluid.step();
  printField(fluid.s);
  fluid.renderD();
}

function draw() {
//  stroke(51);
//  strokeWeight(2);
//
//  let cx = int((0.5 * width) / SCALE);
//  let cy = int((0.5 * height) / SCALE);
//  for (let i = -1; i <= 1; i++) {
//    for (let j = -1; j <= 1; j++) {
//      fluid.addDensity(cx + i, cy + j, random(50, 150));
//    }
//  }
//
//  for (let i = 0; i < 2; i++) {
//    let angle = noise(t) * TWO_PI * 2;
//    let v = p5.Vector.fromAngle(angle);
//    v.mult(0.2);
//    t += 0.01;
//    fluid.addVelocity(cx, cy, v.x, v.y);
//  }
//
//  fluid.step();
//  fluid.renderD();
}
