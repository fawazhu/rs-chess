# RS Chess

Chess in the rs language. Used for training purposes.

## Implementation
This game will run on the web. This will be a multiplayer experience with background sockets being implemented in rust to connect players together, verify their moves, and transmit the information to their web client.

The web clients will use rust webassembly where possible or perhaps a UI toolkit designed for this purpose.

## Goals

 - Learn how rust implemented data structures especially for bit packing.
 - Investigate SIMD optimisations where possible.
 - Understand how to structure a rust project.
 - Understand best practices for organisations of functions and objects in rust.
 - Understand low-latency communication protocols.
 - Understand asynchronous / multithread processing.
 - Understand web options in rust.

