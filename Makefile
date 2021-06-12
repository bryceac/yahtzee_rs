prefix ?= /usr/local
bindir = $(prefix)/bin
SYS := $(shell $(CC) -dumpmachine)

build:
	cargo build --release
install: build
ifneq (, $(findstring darwin, $(SYS)))
	test ! -d $(bindir) && mkdir -p $(bindir)

	install "target/release/yahtzee" "$(bindir)/yahtzee"
else
	install -D "target/release/yahtzee" "$(bindir)/yahtzee"
endif
uninstall:
	rm -rf "$(bindir)/yahtzee"
clean:
	rm -rf target
.PHONY: build install uninstall clean