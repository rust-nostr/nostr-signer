# Use 'VERBOSE=1' to echo all commands, for example 'make help VERBOSE=1'.
ifdef VERBOSE
  Q :=
else
  Q := @
endif

all: cli

help:
	$(Q)echo ""
	$(Q)echo "make                                 - Build binaries files"
	$(Q)echo "make x86_64-unknown-linux-gnu        - Build target x86_64-unknown-linux-gnu"
	$(Q)echo "make precommit                       - Execute precommit steps"
	$(Q)echo "make clean                           - Clean"
	$(Q)echo "make loc                             - Count lines of code in src folder"
	$(Q)echo ""

cli:
	$(Q)cargo build --release

x86_64-unknown-linux-musl:
	$(Q)rustup target add x86_64-unknown-linux-musl
	$(Q)TARGET_CC=x86_64-linux-musl-gcc cargo build --release --target x86_64-unknown-linux-musl

precommit:
	$(Q)cargo fmt && cargo clippy

clean:
	$(Q)cargo clean

loc:
	$(Q)echo "--- Counting lines of .rs files (LOC):" && find src*/ -type f -name "*.rs" -exec cat {} \; | wc -l
