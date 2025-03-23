type Position = {
    x: number;
    y: number;
};

type Velocity = {
    vx: number;
    vy: number;
};

type Entity = {
    position: Position;
    velocity: Velocity;
};

type Style = {
    color: string;
};

type Raft = {
    entity: Entity;
    raft_fighters: RaftFighter[];
    width: number;
    height: number;
    max_health: number;
    curr_health: number;
    style: Style;
};

type Projectile = {
    entity: Entity;
    radius: number;
    style: Style;
};

type RaftFighter = {
    entity: Entity;
    width: number;
    height: number;
    max_health: number;
    curr_health: number;
    style: Style;
}

export type GameState = {
    raft_left: Raft;
    raft_right: Raft;
    projectiles: Projectile[];
    ticks: number;
};

const isPosition = (obj: any): obj is Position => {
    return obj && typeof obj.x === 'number' && typeof obj.y === 'number';
}

const isVelocity = (obj: any): obj is Velocity => {
    return obj && typeof obj.vx === 'number' && typeof obj.vy === 'number';
}

const isEntity = (obj: any): obj is Entity => {
    return obj && isPosition(obj.position) && isVelocity(obj.velocity);
}

const isRaft = (obj: any): obj is Raft => {
    return obj &&
        isEntity(obj.entity) &&
        Array.isArray(obj.raft_fighters) &&
        obj.raft_fighters.every(isRaftFighter) &&
        typeof obj.width === 'number' &&
        typeof obj.height === 'number' &&
        typeof obj.curr_health === 'number' &&
        typeof obj.max_health === 'number';
}

const isProjectile = (obj: any): obj is Projectile => {
    return obj && isEntity(obj.entity) && typeof obj.radius === 'number';
}

const isRaftFighter = (obj: any): obj is RaftFighter => {
    return obj &&
        isEntity(obj.entity) &&
        typeof obj.width === 'number' &&
        typeof obj.height === 'number' &&
        typeof obj.curr_health === 'number' &&
        typeof obj.max_health === 'number';
}

const isGameState = (obj: any): obj is GameState => {
    return obj &&
        isRaft(obj.raft_left) &&
        isRaft(obj.raft_right) &&
        Array.isArray(obj.projectiles) && obj.projectiles.every(isProjectile);
}

export const parseGameState = (jsonString: string): GameState => {
    const parsedData = JSON.parse(jsonString);
    if (isGameState(parsedData)) return parsedData;
    console.error("Parsed data does not conform to GameState structure:", parsedData);
    throw new Error("invalid game state parsed")
}
