.PHONY: run libpatronus.so

check: check.c libpatronus.so
	$(CC) --std=c99 -ggdb -pedantic -Wall -Wextra -Werror -L. -I../extra -lpatronus $< -o $@

libpatronus.so:
	cargo build
	cp ../../target/debug/libpatronus_capi.so libpatronus.so

run: check
	LD_LIBRARY_PATH=. ./check
