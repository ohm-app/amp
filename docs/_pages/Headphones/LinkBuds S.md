---
title: Sony LinkBuds S
layout: post
---

# Sony LinkBuds S

The following documentation is by no means complete. It is a work in progress and gets updated as we
find out more about the protocol.

## Protocol

**Ambient Sound Control**

| Action                  | Type          | Handle | Payload                                                             |
| ----------------------- | ------------- | ------ | ------------------------------------------------------------------- |
| Enable Noise Cancelling | L2CAP Send    | 0x000B | 00000000: 09EF 39FF 5A00 1C40 DB23 EA63 000E 3E0C  ..9.Z..@.#.c..>. |
|                         |               |        | 00000010: 0100 0000 0768 1701 0100 0014 A93C 2640  .....h.......<&@ |
|                         | L2CAP Receive |        | 00000000: 0BEF 1200 FF5A 0009 4010 1D00 319A                        |
|                         |               |        |                                                                     |