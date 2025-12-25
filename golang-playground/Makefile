.PHONY: run build clean escape escape-detail heap-only memory-track test fmt vet all help

# Run the playground
run:
	@echo "==> Running Go Memory Playground..."
	go run .

# Build the binary
build:
	@echo "==> Building binary..."
	go build -o main .

# Clean build artifacts
clean:
	@echo "==> Cleaning..."
	rm -f main
	go clean

# Show escape analysis (basic)
escape:
	@echo "==> Running escape analysis..."
	go build -gcflags="-m" . 2>&1 

# Show detailed escape analysis
escape-detail:
	@echo "==> Running detailed escape analysis..."
	go build -gcflags="-m -m" . 2>&1

# Show only what escapes to heap
heap-only:
	@echo "==> Showing heap escapes only..."
	go build -gcflags="-m" . 2>&1 | grep "escapes to heap"

# Show only memory allocation tracking section
memory-track: run
	@echo ""
	@echo "Note: Run 'make run' to see full output including memory tracking"
