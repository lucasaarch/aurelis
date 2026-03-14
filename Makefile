# Minimal Makefile for daily development tasks

SHELL := /bin/sh

# Tools (can be overridden in environment)
CARGO ?= cargo

.PHONY: help run-dev fmt test

help:
	@printf '%s\n' \
		'Make targets:' \
		'  run-dev   start local services with overmind' \
		'  fmt       cargo fmt --all' \
		'  test      cargo test --workspace'

run-dev:
	overmind start

fmt:
	$(CARGO) fmt --all

test:
	$(CARGO) test --workspace
