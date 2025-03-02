#!/bin/bash

usage() {
    echo 'Usage: ./run.sh <program> <zkvm> <file>'
    echo '<program> - Program name to execute (in programs/)'
    echo '<zkvm>    - sp1/risc0'
    echo '<file>    - filename'
    echo Example: ./run.sh loop-sum risc0
    exit
}

if [ "$#" -ne 3 ]
then
    usage
fi

cargo run --release -p runner -- --prover $2 --program $1 --filename $3
