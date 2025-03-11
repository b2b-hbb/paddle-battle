import { useEffect, useRef, useState } from "react";
import { parseGameState, GameState } from "./wasm-types/wasm-game-state.t"
import init, { WasmState } from "./pkg/paddle_battle"

const canvasWidth = 1000;
const canvasHeight = 500;
// number of ticks executed before querying for input
const TICKS_PER_INPUT = 5;
// fixed size of inner array in Arr<Arr<number>>
const TICK_INPUT_API_CHUNK_SIZE = 10;
// this are the number of ticks wasm will execute with the given input queue
const TICKS_PER_LOOP = 1;
// this sets the delay in which requestAnimationFrame triggers a render
// TODO: decouple frames per second rendered from ticks per second simulated. this will enable interpolation through intermediate states
const FPS_IN_MS = 1000 / 60;
// const FPS_IN_MS = -1;


// Initialize an object to track the state of each key
const buttonPressed = {
  "d": false,
  "D": false,
  "a": false,
  "A": false,
  "ArrowRight": false,
  "ArrowLeft": false,
  "s": false,
  "S": false,
  "Escape": false,
  "ArrowDown": false,
  "p": false,
  "P": false,
  "z": false,
  "Z": false,
  " ": false,
};

type TKeyButtonPressed = keyof typeof buttonPressed

const PaddleGame: React.FC = () => {
  const canvasRef = useRef<HTMLCanvasElement | null>(null);
  const wasmRef = useRef<WasmState | null>(null);
  const lastDrawTimeRef = useRef(Date.now() - FPS_IN_MS);
  const scaleRef = useRef<[number, number]>([1, 1]);
  const tickCounterRef = useRef<number>(0);
  const [gameState, setGameState] = useState<GameState | undefined>(undefined)
  // const gameStateRef = useRef<GameState>();

  function drawGame(ctx: CanvasRenderingContext2D, gameState: GameState) {
    const [scaleX, scaleY] = scaleRef.current;

    ctx.clearRect(0, 0, canvasWidth, canvasHeight);

    ctx.fillStyle = '#0000FF';
    const raftLeft = gameState.raft_left;
    ctx.fillRect(
      raftLeft.entity.position.x * scaleX,
      raftLeft.entity.position.y * scaleY,
      raftLeft.width * scaleX,
      raftLeft.height * scaleY
    );

    const raftRight = gameState.raft_right;
    ctx.fillRect(
      raftRight.entity.position.x * scaleX,
      raftRight.entity.position.y * scaleY,
      raftRight.width * scaleX,
      raftRight.height * scaleY
    );

    ctx.fillStyle = '#008000';
    gameState.projectiles.forEach((projectile) => {
      ctx.beginPath();
      ctx.arc(
        projectile.entity.position.x * scaleX,
        projectile.entity.position.y * scaleY,
        projectile.radius * Math.min(scaleX, scaleY), // Applying scaleX for radius assuming uniform scaling
        0,
        2 * Math.PI
      );
      ctx.fill();
    });

    gameState.raft_left.raft_fighters.forEach((fighter) => {
      ctx.fillStyle = '#0F002F';
      ctx.fillRect(
        fighter.entity.position.x * scaleX,
        fighter.entity.position.y * scaleY,
        fighter.width * scaleX,
        fighter.height * scaleY
      );
    });
    gameState.raft_right.raft_fighters.forEach((fighter) => {
      ctx.fillStyle = '#0F002F';
      ctx.fillRect(
        fighter.entity.position.x * scaleX,
        fighter.entity.position.y * scaleY,
        fighter.width * scaleX,
        fighter.height * scaleY
      );
    });
  }

  const gameLoop = () => {
    if (!wasmRef.current || !canvasRef.current) {
      // console.log("no wasmref or canvasref in game loop")
      // return;
      throw new Error("missing wasmref or canvasref in game loop");
    }

    const now = Date.now();
    const elapsed = now - lastDrawTimeRef.current;

    if (elapsed > FPS_IN_MS) {
      lastDrawTimeRef.current = now - (elapsed % FPS_IN_MS);

      const ctx = canvasRef.current.getContext('2d');
      if (!ctx) {
        // console.log("no ctx in game loop")
        // return;
        throw new Error("missing canvas in game loop")
      }
      ctx.setTransform(1, 0, 0, -1, 0, canvasRef.current.height);

      const inputCodes: number[] = [];

      // TODO: iterate over keys of buttonPressed with TKeyButtonPressed and leverage a switch case with typescript ensuring the check is exhaustive
      // this will ensure keypresses are maintained correctly
      if (buttonPressed["s"] || buttonPressed["S"]) inputCodes.push(0);
      if (buttonPressed["d"] || buttonPressed["D"]) inputCodes.push(1);
      if (buttonPressed["a"] || buttonPressed["A"]) inputCodes.push(2);
      if (buttonPressed["ArrowDown"]) inputCodes.push(3);
      if (buttonPressed["ArrowRight"]) inputCodes.push(4);
      if (buttonPressed["ArrowLeft"]) inputCodes.push(5);
      if (buttonPressed["p"] || buttonPressed["P"]) inputCodes.push(6);
      if (buttonPressed["z"] || buttonPressed["Z"]) inputCodes.push(7);
      if (buttonPressed[" "]) inputCodes.push(8);
      if (buttonPressed["Escape"]) inputCodes.push(86);

      while (inputCodes.length < TICK_INPUT_API_CHUNK_SIZE) {
        inputCodes.push(86);
      }

      let initial_tick = tickCounterRef.current;
      let end_tick = initial_tick + TICKS_PER_LOOP;
      let inputs_needed = TICKS_PER_LOOP / TICKS_PER_INPUT +
        TICKS_PER_LOOP % TICKS_PER_INPUT > 0 ? 1 : 0;

      let final: Array<typeof inputCodes> = [];
      for (let i = 0; i < inputs_needed; i++) {
        final.push(inputCodes)
      }

      const array = new Uint32Array(final.flat());
      const state = parseGameState(wasmRef.current.tick_and_return_state(TICKS_PER_LOOP, array))
      setGameState(state);
      // console.log(state.raft_left)
      drawGame(ctx, state);
      tickCounterRef.current = end_tick;
    }

    requestAnimationFrame(gameLoop);
  };


  const handleKeyDown = (event: KeyboardEvent) => {
    if (event.key in buttonPressed) {
      buttonPressed[event.key as TKeyButtonPressed] = true;
    }
  };

  const handleKeyUp = (event: KeyboardEvent) => {
    if (event.key in buttonPressed) {
      buttonPressed[event.key as TKeyButtonPressed] = false;
    }
  };

  useEffect(() => {
    const loadGame = async () => {
      const canvas = canvasRef.current;
      const wasm = wasmRef.current;

      if (wasm !== null) {
        // console.log("wasm already initiated")
        // return;
        throw new Error("wasm already initiated")
      }

      await init();
      wasmRef.current = new WasmState();

      if (canvas === null) throw new Error("no canvas")

      const ctx = canvas.getContext('2d');
      if (ctx === null) throw new Error("no context")

      const maxX = wasmRef.current.get_max_x();
      const maxY = wasmRef.current.get_max_y();
      const wasmTicksPerInput = wasmRef.current.ticks_per_input();
      const wasmChunkSize = wasmRef.current.tick_input_api_chunk_size();

      if (wasmTicksPerInput !== TICKS_PER_INPUT) throw new Error("wrong ticks per input")
      if (wasmChunkSize !== TICK_INPUT_API_CHUNK_SIZE) throw new Error("wrong chunk size")

      const scaleX = canvasWidth / maxX;
      const scaleY = canvasHeight / maxY;
      scaleRef.current = [scaleX, scaleY];

      window.addEventListener('keydown', handleKeyDown);
      window.addEventListener('keyup', handleKeyUp);

      requestAnimationFrame(gameLoop);
    };

    loadGame();

    return () => {
      window.removeEventListener('keydown', handleKeyDown);
      window.removeEventListener('keyup', handleKeyUp);
    };
  }, []);

  return (
    <div>
      <canvas style={{ border: "5px solid black", margin: "1em" }} ref={canvasRef} width={canvasWidth} height={canvasHeight} />
      <p>
        <br />
        Paddle Left
        <br />
        Max health: {gameState?.raft_left.max_health}
        <br />
        Curr health: {gameState?.raft_left.curr_health}
        <br />
        <br />
        Paddle Right
        <br />
        Max health: {gameState?.raft_right.max_health}
        <br />
        Curr health: {gameState?.raft_right.curr_health}
      </p>
    </div>
  );
}

export default PaddleGame;
