Project 4 - Food Craft, Rust Galaxy
Date: 05/12/2021
Authors: Graham Swain
         Mohammed Bukhattala

This program is a multi-threaded application that performs a deadlock free implementation of the Food
Craft problem. The problem requires multiple threads to be running concurrently and effectively sharing
data between each other. In the program, the threads created are foreman, messenger(s), and miner(s), and
the dock is used as shared memory.

command line arguments:
    1. The amount of time (in seconds) before the driver ends the distribution operation.
    2. 'T' or 'F'. 'T' to write the output to a single file, 'F' to write output to the console.

Usage:
    - cargo run [amount of time] [T or F]

Notes:
    - The first argument must be a unsigned integer ( >= 0).

Errors:
    - No known errors.