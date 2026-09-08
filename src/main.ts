import { invoke } from "@tauri-apps/api/core";
import Matter from 'matter-js';

// module aliases
var Engine = Matter.Engine,
    Render = Matter.Render,
    Runner = Matter.Runner,
    Bodies = Matter.Bodies,
    Composite = Matter.Composite;

// create an engine
var engine = Engine.create();

const width = window.innerWidth;
const height = window.innerHeight;

// create a renderer
var render = Render.create({
    element: document.body,
    engine: engine,
    options:{
        background: 'transparent',
        wireframeBackground: 'transparent',
        height: height,
        width: width
    }
});

// create two boxes and a ground
var boxA = Bodies.rectangle(400, 200, 80, 80);
var boxB = Bodies.rectangle(450, 50, 80, 80);
const groundThickness = 60;
const ground = Bodies.rectangle(
    width / 2, 
    height - (groundThickness / 2), 
    width, 
    groundThickness, 
    { isStatic: true }
);

// add all of the bodies to the world
Composite.add(engine.world, [boxA, boxB, ground]);

// run the renderer
Render.run(render);

// create runner
var runner = Runner.create();

// run the engine
Runner.run(runner, engine);

Matter.Events.on(engine, 'afterUpdate', () => {
  syncInteractiveRegions(Composite.allBodies(engine.world));
});

function syncInteractiveRegions(ballBodies: Matter.Body[]) {
  const regions = ballBodies.map((ballBody) => {
    const { min, max } = ballBody.bounds;
    return {
      x: min.x,
      y: min.y,
      width: max.x - min.x,
      height: max.y - min.y
    };
  });

  invoke('update_regions', { regions }).catch(console.error);
}
