package main

// This file demonstrates Go's escape analysis
// Run with: go build -gcflags="-m" to see escape analysis

// Example 1: Does NOT escape - stays on stack
func noEscape() {
	x := 42
	_ = x
	// x is not returned or referenced outside, stays on stack
}

// Example 2: ESCAPES to heap - pointer returned
func escapesViaReturn() *int {
	x := 42
	return &x // x escapes to heap because pointer is returned
}

// Example 3: ESCAPES to heap - assigned to interface
var globalInterface interface{}

func escapesViaInterface() {
	x := 42
	globalInterface = x // x escapes because it's assigned to interface
}

// Example 4: ESCAPES to heap - too large for stack
func escapesViaSizeTooLarge() {
	// Large arrays typically escape to heap
	largeArray := make([]int, 1000000)
	_ = largeArray
}

// Example 5: Does NOT escape - pointer used locally only
func noEscapeLocalPointer() {
	x := 42
	ptr := &x
	_ = *ptr
	// ptr never leaves this function, so x stays on stack
}

// Example 6: ESCAPES - stored in global
var globalPtr *int

func escapesViaGlobal() {
	x := 42
	globalPtr = &x // x escapes because it's stored in global
}

// Example 7: ESCAPES - closure captures variable
func escapesViaClosure() func() int {
	x := 42
	return func() int {
		return x // x escapes because closure outlives the function
	}
}

// Helper function to demonstrate escape analysis
func DemonstrateEscapeAnalysis() {
	// Run: go build -gcflags="-m" to see escape analysis output

	noEscape()
	ptr := escapesViaReturn()
	_ = *ptr

	escapesViaInterface()
	escapesViaSizeTooLarge()
	noEscapeLocalPointer()
	escapesViaGlobal()

	fn := escapesViaClosure()
	_ = fn()
}
