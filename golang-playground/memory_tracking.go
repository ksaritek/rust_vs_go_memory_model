package main

import (
	"fmt"
	"runtime"
)

// MemStats helper to track memory allocations
type MemStats struct {
	Before runtime.MemStats
	After  runtime.MemStats
}

func TrackMemory(name string, fn func()) {
	var m MemStats

	// Force GC to get clean baseline
	runtime.GC()
	runtime.ReadMemStats(&m.Before)

	// Run the function
	fn()

	// Read memory stats after
	runtime.ReadMemStats(&m.After)

	// Calculate differences
	allocDiff := m.After.TotalAlloc - m.Before.TotalAlloc
	heapAllocDiff := m.After.HeapAlloc - m.Before.HeapAlloc
	heapObjects := m.After.HeapObjects - m.Before.HeapObjects

	fmt.Printf("\n=== Memory Tracking: %s ===\n", name)
	fmt.Printf("  Total allocated:     %d bytes\n", allocDiff)
	fmt.Printf("  Heap allocated:      %d bytes\n", heapAllocDiff)
	fmt.Printf("  Heap objects added:  %d\n", heapObjects)
	fmt.Printf("  Mallocs:             %d\n", m.After.Mallocs-m.Before.Mallocs)
}

// Example 1: Stack allocation (no heap allocation)
func stackOnlyAllocation() {
	x := 42
	y := x + 10
	_ = y
}

// createLargeObject - used for heap allocation demonstration
func createLargeObject(id int) *LargeObject {
	return &LargeObject{
		ID:   id,
		Data: make([]byte, 1024),
	}
}

// Example 2: Heap allocation via pointer return
func heapAllocationViaPointer() {
	objects := make([]*LargeObject, 10)
	for i := 0; i < 10; i++ {
		objects[i] = createLargeObject(i) // Keep references to prevent optimization
	}
	_ = objects // Use it so it doesn't get optimized away
}

// Example 3: Small struct that stays on stack
func stackStructAllocation() {
	type SmallStruct struct {
		A int
		B int
	}
	
	sum := 0
	for i := 0; i < 100; i++ {
		s := SmallStruct{A: i, B: i * 2}
		sum += s.A + s.B // Actually use it so compiler can't optimize away
	}
	_ = sum // Prevent "declared but not used" error
}

// Example 4: Large allocation
func largeAllocation() {
	_ = make([]byte, 1024*1024) // 1MB
}

// Demonstrate memory tracking
func DemonstrateMemoryTracking() {
	fmt.Println("\n" + "============================================================")
	fmt.Println("MEMORY ALLOCATION TRACKING")
	fmt.Println("============================================================")

	// Track stack-only allocation
	TrackMemory("Stack Only (should be minimal)", func() {
		stackOnlyAllocation()
	})

	// Track small struct (might stay on stack)
	TrackMemory("Small Structs on Stack", func() {
		stackStructAllocation()
	})

	// Track heap allocation via pointer return
	TrackMemory("Heap Allocation (createLargeObject x10)", func() {
		heapAllocationViaPointer()
	})

	// Track large allocation
	TrackMemory("Large Allocation (1MB slice)", func() {
		largeAllocation()
	})

	fmt.Println("\n" + "============================================================")
}
