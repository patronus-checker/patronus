# Variables
prefix = /usr/local
exec_prefix = $(prefix)
libdir = $(prefix)/lib
includedir = $(prefix)/include
pkgconfigdir = $(libdir)/pkgconfig
pkglibdir = $(libdir)/patronus

srcdir = .
DESTDIR =

CARGO = cargo
INSTALL = install
CBINDGEN = cbindgen
CARGO_TARGET_DIR = ./target
DEBUG = 1

ifneq ($(DEBUG),"1")
	CARGO_RELEASE_ARGS =
	RUST_TARGET_SUBDIR = debug
else
	CARGO_RELEASE_ARGS = --release
	RUST_TARGET_SUBDIR = release
endif

version = $(shell grep version Cargo.toml | tr -cd 0-9.)
abiversion = 0

# Library
RUST_SRC = \
	Cargo.toml \
	src/lib.rs

libpatronus.so: $(RUST_SRC)
	CARGO_TARGET_DIR=$(CARGO_TARGET_DIR) \
	PATRONUS_PROVIDER_DIR=$(pkglibdir) \
	$(CARGO) build --manifest-path Cargo.toml $(CARGO_RELEASE_ARGS) && \
	cp $(CARGO_TARGET_DIR)/${RUST_TARGET_SUBDIR}/libpatronus_capi.so $@

patronus.pc: Cargo.toml
	echo 'libdir=${libdir}' > $@
	echo 'includedir=${includedir}' >> $@
	echo >> $@
	echo 'Name: libpatronus' >> $@
	echo 'Description: ${description}' >> $@
	echo 'Version: ${version}' >> $@
	echo 'Libs: -L$${libdir} -lpatronus' >> $@
	echo 'Cflags: -I$${includedir}/patronus' >> $@

# Headers
patronus/patronus.h: libpatronus.so cbindgen.toml
	CARGO_TARGET_DIR=$(CARGO_TARGET_DIR) \
	$(CBINDGEN) $(srcdir) -o $@

# Examples
checker: examples/checker.c libpatronus.so patronus/patronus.h
	$(CC) --std=c99 -ggdb -pedantic -Wall -Wextra -Werror -L. -I. -lpatronus $< -o $@

examples: checker

# Common
all: libpatronus.so patronus/patronus.h patronus.pc examples

install: all
	$(INSTALL) -d -m 755 $(DESTDIR)$(libdir)
	$(INSTALL) -m 755 libpatronus.so $(DESTDIR)$(libdir)/libpatronus.so.1.$(abiversion)
	ln -fs libpatronus.so.1.$(abiversion) $(DESTDIR)$(libdir)/libpatronus.so.1
	ln -fs libpatronus.so.1.$(abiversion) $(DESTDIR)$(libdir)/libpatronus.so
	$(INSTALL) -d -m 755 $(DESTDIR)$(pkgconfigdir)
	$(INSTALL) -m 644 patronus.pc $(DESTDIR)$(pkgconfigdir)
	$(INSTALL) -d -m 755 $(DESTDIR)$(includedir)/patronus
	$(INSTALL) -m 644 patronus/patronus.h $(DESTDIR)$(includedir)/patronus/patronus.h

clean:
	rm -f checker libpatronus.so patronus/patronus.h patronus.pc
	CARGO_TARGET_DIR=$(CARGO_TARGET_DIR) \
	$(CARGO) clean
