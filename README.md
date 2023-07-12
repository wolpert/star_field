# Star Field

## Purpose

This project was started as a way to learn how to write games with Rust
using the ECS-style game engine Bevy.

## Status

[![Makefile CI](https://github.com/wolpert/star_field/actions/workflows/makefile.yml/badge.svg)](https://github.com/wolpert/star_field/actions/workflows/makefile.yml)

## Building

You need to have rust installed, and core C development utilities like
`make` as well. Just run `make` on the command line to build the app
and run the un-optimized version of the app. 

### Development

Use `make dev` as your default build command as this will
* Run the tests
* Run the linter
* Run the application

### Github Actions

If you change the `.github/workflows/makefile.yml` file, run 'gh act' to
verify the changes via `make act` To install act, see here: https://github.com/nektos/act/blob/master/README.md (I picked the github extension install)

## Gameplay

It's effectively an infinite runner style to be used on your phone with
abstract graphics. ('cause I cannot draw)

## Notes

Random musings here....
