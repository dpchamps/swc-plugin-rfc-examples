# SWC Plugin Module State Examples 

The primary purpose of this repo is to demonstrate the following:

> When authoring plugins, module state for the plugin is expected to be preserved for 
> the lifetime of the compilation.
> We see this when writing babel plugins, as well as when writing visitors directly
> in Rust and using the SWC compiler directly
> However, we do not see module state preserved for SWC plugins

This repo is a companion for the following RFC: (add rfc link here), which addresses the current
limitations, and proposes a solution.

## Overview of examples

There are three main examples, which compile the folder [js-fixtures](./js-fixtures) into a respective 
output directory [within dist](./dist).

1. `npm run babel` [output dir dist/babel](./dist/babel)
   2. Executes babel, configured with the [babel visitor](./src/babel-plugin/ident-counter.js) as a plugin
3. `npm run swc_compile` [output dir dist/swc-compile](./dist/swc-compile)
   4. Executes an swc compilation with the [swc visitor](./crates/ident_counter/src/lib.rs) directly
5. `npm run swc_plugin` [output dir dist/swc-plugin](./dist/swc-plugin)
   6. Executes the swc cli configured with the [swc visitor](./crates/ident_counter_plugin/src/lib.rs) as a plugin


## About the visitor

The visitor is not intended produce working code. It provides output that
demonstrates how module level state is preserved across capabilities.

It does two things:

1. Initialize a random number at the module level, append the number to each ident name visited
2. Hold in memory a count of every identifier visited, append the count to each ident name visited


## Observations

The key observation to make here are that module-level state is preserved across the lifetime 
of the entire compilation for the babel plugin, as well as the direct swc compilation

The swc plugin however produces results that run counter to expectations:

1. each identifier will have a separate unique id, because the random number is genrated per visitor invocation
2. each identifier will start counting upwards per file, because the ident count state is initialized per visitor invocation

## Motivations 

The primary motivations for these examples are to demonstrate the difference in expectations across plugin capabilities.
The RFC outlines why this might be important, which are primarily performance concerns. 