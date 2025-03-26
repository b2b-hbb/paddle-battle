import * as cbor from 'cbor-web';

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

// Type mappings configuration
type TypeName = 'Raft' | 'Entity' | 'Style' | 'Projectile' | 'RaftFighter' | 'Position' | 'Velocity' | 'GunTypes';

type TypeConfig = {
    fields?: readonly string[];
    nestedTypes?: {
        [key: string]: TypeName;
    };
    values?: readonly string[];
};

const TYPE_MAPPINGS: Record<TypeName, TypeConfig> = {
    Raft: {
        fields: ['entity', 'width', 'height', 'max_health', 'curr_health', 'raft_fighters', 'style'],
        nestedTypes: {
            entity: 'Entity',
            style: 'Style',
            raft_fighters: 'RaftFighter'
        }
    },
    Entity: {
        fields: ['position', 'velocity', 'is_active'],
        nestedTypes: {
            position: 'Position',
            velocity: 'Velocity'
        }
    },
    Style: {
        fields: ['color']
    },
    Projectile: {
        fields: ['entity', 'radius', 'style'],
        nestedTypes: {
            entity: 'Entity',
            style: 'Style'
        }
    },
    RaftFighter: {
        fields: ['entity', 'width', 'height', 'gun', 'curr_health', 'max_health', 'style'],
        nestedTypes: {
            entity: 'Entity',
            style: 'Style',
            gun: 'GunTypes'
        }
    },
    Position: {
        fields: ['x', 'y']
    },
    Velocity: {
        fields: ['vx', 'vy']
    },
    GunTypes: {
        values: ['Bazooka', 'SMG', 'FlameThrower', 'StraightShooter']
    }
} as const;

// Helper function to convert Map to plain object
const mapToObject = (map: Map<number, any>, type: TypeName | null = null): any => {
    if (!(map instanceof Map)) return map;
    
    const obj: any = {};
    const typeConfig = type ? TYPE_MAPPINGS[type] : null;
    
    Array.from(map.entries()).forEach(([key, value]) => {
        let fieldName: string;
        
        if (typeConfig?.fields) {
            fieldName = typeConfig.fields[key];
        } else {
            fieldName = String(key);
        }

        if (value instanceof Map) {
            // Determine the type of the nested object based on the field name
            const nestedType = typeConfig?.nestedTypes?.[fieldName] || null;
            obj[fieldName] = mapToObject(value, nestedType);
        } else if (Array.isArray(value)) {
            if (fieldName === 'raft_fighters') {
                // Handle raft_fighters array - each item is a Map that needs to be parsed as RaftFighter
                obj[fieldName] = value.map(item => {
                    if (item instanceof Map) {
                        return mapToObject(item, 'RaftFighter');
                    }
                    return item;
                });
            } else if (fieldName === 'left_projectiles' || fieldName === 'right_projectiles') {
                obj[fieldName] = value.map(item => mapToObject(item, 'Projectile'));
            } else if (fieldName === 'gun') {
                // Handle gun array case - take the first value as the enum variant
                const gunValues = TYPE_MAPPINGS.GunTypes.values;
                if (gunValues) {
                    obj[fieldName] = gunValues[value[0]];
                }
            } else {
                obj[fieldName] = value.map(item => mapToObject(item));
            }
        } else {
            obj[fieldName] = value;
        }
    });
    return obj;
};

export const parseGameState = (data: Uint8Array): GameState => {
    try {
        const decoded = cbor.decode(data);
        
        // Convert Map to plain object
        const parsedData = {
            raft_left: mapToObject(decoded.get(0), 'Raft'),
            raft_right: mapToObject(decoded.get(1), 'Raft'),
            left_projectiles: decoded.get(2).map((item: Map<number, any>) => mapToObject(item, 'Projectile')),
            right_projectiles: decoded.get(3).map((item: Map<number, any>) => mapToObject(item, 'Projectile')),
            ticks: decoded.get(4)
        };
        
        if (isGameState(parsedData)) return parsedData;
        throw new Error("invalid game state parsed")
    } catch (error) {
        console.error("Error parsing game state:", error);
        throw error;
    }
}

// Add type declarations for the WebAssembly functions
declare global {
    interface Window {
        wasm: {
            update_game_state: (input: Uint8Array, state: Uint8Array) => Uint8Array;
            get_initial_state: () => Uint8Array;
        }
    }
}
