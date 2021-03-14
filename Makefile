.PHONY: clean build

build: bin/buzz bin/test_buzz

clean: lib bin
	rm -r bin/ lib/

bin:
	mkdir -p bin/

lib:
	mkdir -p lib/

lib/buzz.o: lib
	gcc -c -o lib/buzz.o src/buzz.c

bin/test_buzz: bin lib lib/buzz.o
	gcc -o bin/test_buzz tests/test_buzz.c lib/buzz.o

bin/buzz: bin lib/buzz.o
	gcc -o bin/buzz src/cmd.c lib/buzz.o

test: build
	bin/test_buzz
