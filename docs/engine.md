# Engine Architecture overview

## Goals

 - Fast (play games at 60 fps)
    - Simulation speed needs to be fast, chain can catch up later.
    - Need to have consensus on user inputs
    - Engine can be compiled to native/wasm. can progress quickly trusting server, but verifying using parallel threads
 - No new external validators (ie making an orbit chain)
    - Leverage state channels cosigning for end state and skip reexecution
 - Friendly with multiple networking models
    - P2P counter signatures (with possible relay server for inputs)
    - server client with server authoritative for inputs
    - server client with chain authoritative for inputs


## Constraints

 - Determinism
    - can't use floats - rust corelib uses floats
 - Chain gas limit
    - Checkpointing injection for stylus code (check if gas left is enough to do a checkpoint before reexecution halt)
 - Chain gas price
 - Contract size limit
 - Reexecution logic in case of disagreement between players


## How is this done

 - Core game engine loop can be compiled to WASM and native
    - WASM entrypoints for browser 



## References

 - Arbitrum proving architecture with hybrid native vs virtualised
 - Reexecution proving by OVM 1.0
 - Engines: Bevvy, Godot, MUD
 - 

### TODO

 - assess better store layouts to reduce state fetches and permit cache locality
    - ECS-ish approach reducing number of comparisons by leveraging systems
 - enable softfloat to avoid corelib float lack of determinism
 - 
