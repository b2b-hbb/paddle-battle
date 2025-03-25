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
  "ArrowUp": false,
  "p": false,
  "P": false,
  "z": false,
  "Z": false,
  " ": false,
  "w": false,
};

type TKeyButtonPressed = keyof typeof buttonPressed

const LoadScreen: React.FC<{ onStart: (leftGun: string, rightGun: string, replayInputs?: number[]) => void }> = ({ onStart }) => {
    const [replayInput, setReplayInput] = useState('');

    const handleStart = () => {
        if (replayInput) {
            try {
                const replayInputs = JSON.parse(replayInput);
                // Default guns for replay mode
                onStart('Bazooka', 'SMG', replayInputs);
            } catch (e) {
                alert('Invalid replay input format. Please enter a valid array of numbers, e.g. [0, 5, 4]');
            }
        } else {
            alert('Please enter replay inputs to continue');
        }
    };

    return (
        <div style={{ 
            display: 'flex', 
            flexDirection: 'column', 
            alignItems: 'center', 
            gap: '20px',
            padding: '20px' 
        }}>
            <h2>Paddle Battle Replay Mode</h2>
            <div style={{ width: '100%', maxWidth: '500px' }}>
                <label style={{ display: 'block', marginBottom: '10px' }}>Enter Replay Inputs:</label>
                <textarea 
                    value={replayInput} 
                    onChange={(e) => setReplayInput(e.target.value)}
                    placeholder="Enter array of numbers, e.g. [0, 5, 4]"
                    style={{
                        width: '100%',
                        height: '100px',
                        padding: '10px',
                        marginBottom: '10px',
                        fontFamily: 'monospace'
                    }}
                />
            </div>
            <button 
                onClick={handleStart}
                style={{
                    padding: '10px 20px',
                    fontSize: '16px',
                    cursor: 'pointer'
                }}
            >
                Start Replay
            </button>
        </div>
    );
};

const PaddleGame: React.FC = () => {
  const canvasRef = useRef<HTMLCanvasElement | null>(null);
  const wasmRef = useRef<WasmState | null>(null);
  const lastDrawTimeRef = useRef(Date.now() - FPS_IN_MS);
  const scaleRef = useRef<[number, number]>([1, 1]);
  const tickCounterRef = useRef<number>(0);
  const [gameState, setGameState] = useState<GameState | undefined>(undefined)
  const [gameStarted, setGameStarted] = useState(false);
  const [leftGun, setLeftGun] = useState('Bazooka');
  const [rightGun, setRightGun] = useState('SMG');
  const replayInputsRef = useRef<number[] | undefined>(undefined);
  const replayIndexRef = useRef<number>(0);

  function drawGame(ctx: CanvasRenderingContext2D, gameState: GameState) {
    console.log('drawGame called');
    console.log('Current transformation matrix:', ctx.getTransform());
    const [scaleX, scaleY] = scaleRef.current;

    ctx.clearRect(0, 0, canvasWidth, canvasHeight);

    const raftLeft = gameState.raft_left;
    console.log('Raft Left Color:', raftLeft.style.color);
    ctx.fillStyle = raftLeft.style.color;
    ctx.fillRect(
      raftLeft.entity.position.x * scaleX,
      raftLeft.entity.position.y * scaleY,
      raftLeft.width * scaleX,
      raftLeft.height * scaleY
    );

    const raftRight = gameState.raft_right;
    console.log('Raft Right Color:', raftRight.style.color);
    ctx.fillStyle = raftRight.style.color;
    ctx.fillRect(
      raftRight.entity.position.x * scaleX,
      raftRight.entity.position.y * scaleY,
      raftRight.width * scaleX,
      raftRight.height * scaleY
    );

    [...gameState.left_projectiles, ...gameState.right_projectiles].forEach((projectile) => {
      const { radius, style } = projectile;
      console.log('Projectile Color:', style.color);
      ctx.fillStyle = style.color;
      ctx.beginPath();
      ctx.arc(
        projectile.entity.position.x * scaleX,
        projectile.entity.position.y * scaleY,
        radius * Math.min(scaleX, scaleY),
        0,
        2 * Math.PI
      );
      ctx.fill();
    });

    gameState.raft_left.raft_fighters.forEach((fighter) => {
      console.log('Raft Left Fighter Color:', fighter.style.color);
      ctx.fillStyle = fighter.style.color;
      ctx.fillRect(
        fighter.entity.position.x * scaleX,
        fighter.entity.position.y * scaleY,
        fighter.width * scaleX,
        fighter.height * scaleY
      );
      ctx.fillStyle = '#000000';
      ctx.fillText(`HP: ${fighter.curr_health}`,
        fighter.entity.position.x * scaleX + 50,
        (fighter.entity.position.y - 10) * scaleY
      );
    });
    gameState.raft_right.raft_fighters.forEach((fighter) => {
      console.log('Raft Right Fighter Color:', fighter.style.color);
      ctx.fillStyle = fighter.style.color;
      ctx.fillRect(
        fighter.entity.position.x * scaleX,
        fighter.entity.position.y * scaleY,
        fighter.width * scaleX,
        fighter.height * scaleY
      );
      ctx.fillStyle = '#000000';
      ctx.fillText(`HP: ${fighter.curr_health}`,
        fighter.entity.position.x * scaleX + 50,
        (fighter.entity.position.y - 10) * scaleY
      );
    });
  }

  const gameLoop = () => {
    if (!wasmRef.current || !canvasRef.current) {
      throw new Error("missing wasmref or canvasref in game loop");
    }
    
    const now = Date.now();
    const elapsed = now - lastDrawTimeRef.current;
    
    if (elapsed > FPS_IN_MS) {
      // Check if we're in replay mode and have exhausted all inputs
      if (replayInputsRef.current && replayIndexRef.current >= replayInputsRef.current.length) {
        return; // Stop execution completely
      }
      lastDrawTimeRef.current = now - (elapsed % FPS_IN_MS);

      const ctx = canvasRef.current.getContext('2d');
      if (!ctx) {
        throw new Error("missing canvas in game loop");
      }
      ctx.setTransform(1, 0, 0, -1, 0, canvasRef.current.height);

      let inputCodes: number[] = [];

      if (replayInputsRef.current) {
        // In replay mode, use the pre-recorded inputs
        if (replayIndexRef.current < replayInputsRef.current.length) {
          inputCodes.push(replayInputsRef.current[replayIndexRef.current]);
          replayIndexRef.current++;
        }
      } else {
        // Normal gameplay mode
        if (buttonPressed["s"] || buttonPressed["S"]) inputCodes.push(0);
        if (buttonPressed["d"] || buttonPressed["D"]) inputCodes.push(1);
        if (buttonPressed["a"] || buttonPressed["A"]) inputCodes.push(2);
        if (buttonPressed["ArrowDown"]) inputCodes.push(3);
        if (buttonPressed["ArrowRight"]) inputCodes.push(4);
        if (buttonPressed["ArrowLeft"]) inputCodes.push(5);
        if (buttonPressed["p"] || buttonPressed["P"]) inputCodes.push(6);
        if (buttonPressed["z"] || buttonPressed["Z"]) inputCodes.push(7);
        if (buttonPressed[" "]) inputCodes.push(8);
        if (buttonPressed["ArrowUp"]) inputCodes.push(9);
        if (buttonPressed["ArrowDown"]) inputCodes.push(10);
        if (buttonPressed["w"]) inputCodes.push(11);
        if (buttonPressed["s"]) inputCodes.push(12);
        if (buttonPressed["Escape"]) inputCodes.push(86);
      }

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
      drawGame(ctx, state);
      tickCounterRef.current = end_tick;
    }

    console.log({
      here: "here",
      replayInputs: replayInputsRef.current,
      replayIndex: replayIndexRef.current,
      replayInputsLength: replayInputsRef.current ? replayInputsRef.current.length : 0
    });

    // Only continue the animation frame if we're not in replay mode or still have inputs left
    if (!replayInputsRef.current || replayIndexRef.current < replayInputsRef.current.length) {
      requestAnimationFrame(gameLoop);
    }
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

  const startGame = (leftGun: string, rightGun: string, replayInputs?: number[]) => {
    if (!gameStarted) {
      setLeftGun(leftGun);
      setRightGun(rightGun);
      replayInputsRef.current = replayInputs;
      replayIndexRef.current = 0;
      setGameStarted(true);
    }
  };

  useEffect(() => {
    console.log('useEffect triggered');
    if (!gameStarted) return;

    const loadGame = async () => {
      const canvas = canvasRef.current;
      console.log('Canvas ref:', canvas);

      if (wasmRef.current !== null) {
        throw new Error("wasm already initiated");
      }

      await init();
      wasmRef.current = new WasmState();

      if (canvas === null) throw new Error("no canvas");

      const ctx = canvas.getContext('2d');
      if (ctx === null) throw new Error("no context");

      const maxX = wasmRef.current.get_max_x();
      const maxY = wasmRef.current.get_max_y();
      const wasmTicksPerInput = wasmRef.current.ticks_per_input();
      const wasmChunkSize = wasmRef.current.tick_input_api_chunk_size();

      if (wasmTicksPerInput !== TICKS_PER_INPUT) throw new Error("wrong ticks per input");
      if (wasmChunkSize !== TICK_INPUT_API_CHUNK_SIZE) throw new Error("wrong chunk size");

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
  }, [gameStarted]);

  return (
    <div>
      {!gameStarted ? (
        <LoadScreen onStart={startGame} />
      ) : (
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
      )}
    </div>
  );
}

export default PaddleGame;
