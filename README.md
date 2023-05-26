## Description

This is a small application that will run in the background and stores all the keystrokes events in a database.

Consumer repository: https://github.com/krshrimali/keystroke-consumer-rs

## Plan

1. Figure out:
    * How to capture key stroke events.
    * What kind of key stroke events are there?
    * Which ones to store?
2. Print the key stroke events.
3. Store the key stroke events in a text file.
4. Run the application in the background
5. Integrate postgres DB in Rust
6. Write to the DB:
    * batch / single?
    * when to write?
        * every time a keystroke event is pressed (definite no)
        * frequency dependent on number of keystrokes or time elapsed? (siding towards number of keystrokes available)

## North Pole

- Benchmark DB calls + DB memory usage based on different windows/styles/optimizations

## Motivation

The goal is to just explore the DB side in Rust, and try optimizing myself.
