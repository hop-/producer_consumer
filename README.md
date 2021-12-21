# producer_consumer

A solution for producer consumer problem.

github repo <https://github.com/hop-/producer-consumer>

Author:
    Hovhannes Palyan (hovhannespalyan@gmail.com)

## The Problem

An application which has `N` producers where `N` is between 1 and 10, `M` consumers where `M` is between 1 and 10 and a data queue. Each producer and consumer is a separate thread and all threads are working concurrently.

Producer thread sleeps randomly from 1 to 100 milliseconds then wakes up and puts a random number between 1 and 100 to the data queue.

Consumer thread sleeps randomly from 1 to 100 milliseconds then wakes up and takes the number from the queue and saves it to the output `data.txt` file.
All numbers are appended in the file and comma separated.

When producer thread puts the next number to data queue it checks the size of data queue and if it is greater or equal to 100 the producer thread is blocked until the number of elements gets below 80.

When consumer thread wants to take the next number from data queue and no elements in it, consumer thread is blocked until new element is added to data queue by a producer.

When we start the application we need to insert the `N` and the `M` then program starts all threads.
It should print current number of elements of data queue in each second.

When we stop program it should interrupt all producers and wait untill all consumers save all queued data.

## The Solotion

This is a solution to the producer consumer written in Rust programing language.

### Requirements

The project has been tested on linux machines.

Rust/Cargo installed

### Build

Use Cargo to build the solution run `cargo build --release` command in root of the project.
Check the `target` direcotry.

### Run

Execute the solution.

After the start-up please input the number of producers and consumers. The numbers should be between 1 to 10.

Iterruption (`Ctrl+C`) of the process will gracefully shut down the process.
It will stop producers and wait till consumers complete their jobs.

### Output

The solution prints current number of elements of data queue each second.
It also generates output file `data.txt` which contains the values of consumed element.
