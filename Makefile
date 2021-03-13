.PHONY: clean all

all: bin/buzz

clean: lib bin
	rm -r bin/ lib/

bin:
	mkdir -p bin/

lib:
	mkdir -p lib/

lib/buzz.o: lib
	gcc -c -o lib/buzz.o src/buzz.c

bin/buzztest: bin lib lib/buzz.o
	gcc -o bin/buzztest tests/test_buzz.c lib/buzz.o

bin/buzz: bin lib/buzz.o
	gcc -o bin/buzz src/cmd.c lib/buzz.o

test: bin/buzztest
	bin/buzztest
