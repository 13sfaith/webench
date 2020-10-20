# Systems Assignment - webench

A simple HTTP request/benchmark CLI written in rust.

## Motiviation

This is a project I built for the Cloudflare systems intern assesment. The basic working verison of the software took me 2 weekends to put together. The challenge required no libraries be used for the actual handling of the HTTP which brought about several issues.

## Features

There are three commands that you can use on this tool:

- --url <url string> (used to define what domain to send the request to)
- --profile <pos int> *optional* (if this flag is used enters benchmark mode)
- --help (displays a similar message to this README in the command line)

The standard mode (when profile is not engaged) returns the response given by the domain specified using the url tag. The benchmark mode (when profile is engaged) returns a set of data based off of a set number of requests to the domain.

These data points include (as instructed by the cloudflare assesment):
 
-    The number of requests
-    The fastest time
-    The slowest time
-    The mean & median times
-    The percentage of requests that succeeded
-    Any error codes returned that weren't a success
-    The size in bytes of the smallest response
-    The size in bytes of the largest response

## Using the tool

**Compiling**
This tool utilizes cargo and as such has a very intuitive compilation process

*Testing*
In order to test the tool without compiling out a shippable binary in the main folder simply run:

```BASH
cargo run -- <commands>
```

*Building*
In order to build the tool for deployment in the main folder run:

```BASH
cargo install
```

## Few notes

I really enjoyed making this tool and I hope you enjoy using it. Just a couple things before I wrap up this readme.

1. The pictures of this tool running on my system are located in ./screen-shots
2. This CLI does not support https requests. I did attempt to make a tool that could do that however without libraries it turned into quite the task (one I do hope to finish soon but not with this submission)
3. Rust does come with a system for generating its own documentation based on certains kinds of comments ('///'). Since this was my first time using Rust I did not want to rely on something so foreign to me for something so important, another thing to learn in side projects!
4. Thank you for taking the time read this all the way through I do sincerly appreacite your time. 
