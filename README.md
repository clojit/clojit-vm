Clojit-VM
======================

## What

Clojure Virtual Maschine written in Rust

## Why

Explore how to write a VM and JIT in Rust

## How


    chmod +x clojitvm
    ./clojitvm resources/programs/let_test.clj
  
 
This first runs the Clojure->Clojit Bytecode comipler (written in Clojure).

Then runs 'cargo run' to pass in the Bytecode file to VM.