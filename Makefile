prog :=podcli

debug ?=

$(info debug is $(debug))

ifdef debug
  release :=
  target :=debug
  extension :=debug
else
  release :=--release
  target :=release
  extension :=
endif

ifdef extension
	executable :=$(prog)-$(extension)
else
	executable :=$(prog)
endif

build:
	cargo build $(release)

install:
	cp target/$(target)/$(prog) ~/.local/bin/$(executable)


all: build install

## help: Show help
.PHONY: help
help: Makefile
	@echo "usage: make $(prog) [debug=1]"
	@sed -n 's/^##//p' $<

