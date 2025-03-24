type Position = {
    x: number;  // u32 in Rust
    y: number;  // u32 in Rust
};

type Velocity = {
    vx: number;  // i32 in Rust
    vy: number;  // i32 in Rust
};

type Entity = {
    position: Position;
    velocity: Velocity;
    is_active: boolean;
};

type Style = {
    color: string;
};

type GunTypes = "Bazooka" | "SMG" | "FlameThrower" | "StraightShooter";

type Raft = {
    entity: Entity;
    raft_fighters: RaftFighter[];
    width: number;  // u32 in Rust
    height: number;  // u32 in Rust
    max_health: number;  // u32 in Rust
    curr_health: number;  // u32 in Rust
    style: Style;
};

type Projectile = {
    entity: Entity;
    radius: number;  // u32 in Rust
    style: Style;
};

type RaftFighter = {
    entity: Entity;
    width: number;  // u32 in Rust
    height: number;  // u32 in Rust
    gun: GunTypes;
    max_health: number;  // u32 in Rust
    curr_health: number;  // u32 in Rust
    style: Style;
}

export type GameState = {
    raft_left: Raft;
    raft_right: Raft;
    left_projectiles: Projectile[];
    right_projectiles: Projectile[];
    ticks: number;  // u32 in Rust
};

const isPosition = (obj: any): obj is Position => {
    return obj && typeof obj.x === 'number' && typeof obj.y === 'number';
}

const isVelocity = (obj: any): obj is Velocity => {
    return obj && typeof obj.vx === 'number' && typeof obj.vy === 'number';
}

const isEntity = (obj: any): obj is Entity => {
    return obj && isPosition(obj.position) && isVelocity(obj.velocity) && typeof obj.is_active === 'boolean';
}

const isGunTypes = (obj: any): obj is GunTypes => {
    return obj && ["Bazooka", "SMG", "FlameThrower", "StraightShooter"].includes(obj);
}

const isRaft = (obj: any): obj is Raft => {
    return obj &&
        isEntity(obj.entity) &&
        Array.isArray(obj.raft_fighters) &&
        obj.raft_fighters.every(isRaftFighter) &&
        typeof obj.width === 'number' &&
        typeof obj.height === 'number' &&
        typeof obj.curr_health === 'number' &&
        typeof obj.max_health === 'number' &&
        isStyle(obj.style);
}

const isProjectile = (obj: any): obj is Projectile => {
    return obj && isEntity(obj.entity) && typeof obj.radius === 'number' && isStyle(obj.style);
}

const isRaftFighter = (obj: any): obj is RaftFighter => {
    return obj &&
        isEntity(obj.entity) &&
        typeof obj.width === 'number' &&
        typeof obj.height === 'number' &&
        isGunTypes(obj.gun) &&
        typeof obj.curr_health === 'number' &&
        typeof obj.max_health === 'number' &&
        isStyle(obj.style);
}

const isStyle = (obj: any): obj is Style => {
    return obj && typeof obj.color === 'string';
}

const isGameState = (obj: any): obj is GameState => {
    return obj &&
        isRaft(obj.raft_left) &&
        isRaft(obj.raft_right) &&
        Array.isArray(obj.left_projectiles) && obj.left_projectiles.every(isProjectile) &&
        Array.isArray(obj.right_projectiles) && obj.right_projectiles.every(isProjectile) &&
        typeof obj.ticks === 'number';
}

export const parseGameState = (jsonString: string): GameState => {
    const parsedData = JSON.parse(jsonString);
    if (isGameState(parsedData)) return parsedData;
    console.error("Parsed data does not conform to GameState structure:", parsedData);
    throw new Error("invalid game state parsed")
}
