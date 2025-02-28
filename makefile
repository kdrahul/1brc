# Makefile for the 1BRC project

# Define the executable name
EXECUTABLE = 1brc

# Define the Go source file
SOURCE = main.go

# Compiler flags (optional, can be used to optimize the build)
GOFLAGS = -ldflags="-s -w" # Reduce binary size

# CPU profile output file
PROFILE = cpu.prof

# Input file (adjust if needed)
INPUT_FILE = measurements_1billion.txt
# Flag to set the revision number
REVISION_FLAG = -revision=1

# Default target: build
all: build

# Build the executable
build:
	go build $(GOFLAGS) -o $(EXECUTABLE) $(SOURCE)

# Run the benchmark with CPU profiling
run: build
	time ./$(EXECUTABLE) -cpuprofile=$(PROFILE)

# Clean the project: remove the executable and profile file
clean:
	rm -f $(EXECUTABLE) $(PROFILE)

# Run the program and print the revision number (for validation)
run_print_revision: build
	./$(EXECUTABLE) $(REVISION_FLAG) $(INPUT_FILE)

# Help target: show available commands
help:
	@echo "Available commands:"
	@echo "  make build      - Build the executable"
	@echo "  make run        - Run the benchmark with CPU profiling"
	@echo "  make clean      - Remove the executable and profile file"
	@echo "  make run_print_revision - Runs the program and shows the output"
	@echo "  make help       - Show this help message"
