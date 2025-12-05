SOURCE = solution.rs
BINARY = solution

build:
	rustc $(SOURCE) -o $(BINARY)

run: build
	./$(BINARY) sample.txt
	$(MAKE) clean

test: build
	./$(BINARY) input.txt
	$(MAKE) clean

clean:
	rm -f $(BINARY)

.PHONY: build run test clean
