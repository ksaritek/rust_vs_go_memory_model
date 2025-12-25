package main

import "fmt"

func main() {
	fmt.Println("=== Go Memory Model Playground ===")
	fmt.Println()

	// Example 1: Stack vs Heap allocation
	fmt.Println("1. Stack vs Heap Allocation")
	stackExample()
	heapExample()

	// Example 2: Pointer sharing (unique to Go - multiple owners!)
	fmt.Println("\n2. Pointer Sharing (Multiple References)")
	pointerSharingExample()

	// Example 3: Slice sharing (shared backing array)
	fmt.Println("\n3. Slice Sharing (Shared Backing Array)")
	sliceSharingExample()

	// Example 4: Escape analysis (what causes heap allocation?)
	DemonstrateEscapeAnalysis()

	// Example 5: Memory tracking (prove it with measurements)
	DemonstrateMemoryTracking()
}

// Stack allocation - variable stays on stack
func stackExample() {
	x := 42 // Lives on stack, automatically cleaned when function returns
	fmt.Printf("  Stack variable x = %d (allocated on stack)\n", x)
}

// Heap allocation - variable escapes to heap
func heapExample() {
	user := createUser() // Escapes to heap
	fmt.Printf("  Heap variable user = %+v (allocated on heap, GC will clean up)\n", user)
}

// This function causes escape to heap because we return a pointer
func createUser() *User {
	u := User{Name: "Alice", Age: 30}
	// Compiler detects: pointer escapes, allocates on heap
	return &u
}

// Multiple pointers can reference the same data - GC tracks references
func pointerSharingExample() {
	user := &User{Name: "Bob", Age: 25}

	ptr1 := user
	ptr2 := user
	ptr3 := user

	fmt.Printf("  Original: %p (ptr at %p) -> %+v\n", user, &user, *user)
	fmt.Printf("  Ptr1:     %p (ptr at %p) -> %+v\n", ptr1, &ptr1, *ptr1)
	fmt.Printf("  Ptr2:     %p (ptr at %p) -> %+v\n", ptr2, &ptr2, *ptr2)
	fmt.Printf("  Ptr3:     %p (ptr at %p) -> %+v\n", ptr3, &ptr3, *ptr3)
	fmt.Println("  All pointers share the same heap memory!")
	fmt.Println("  GC will clean up when all references are gone")

	// Modify through one pointer, affects all
	ptr1.Age = 26
	fmt.Printf("  After modification via ptr1: %+v\n", *user)
}

// Slices can share the same backing array
func sliceSharingExample() {
	original := []int{1, 2, 3, 4, 5}
	slice1 := original[1:4] // Shares backing array
	slice2 := original[2:]  // Also shares backing array

	fmt.Printf("  Original: %v (len=%d, cap=%d)\n", original, len(original), cap(original))
	fmt.Printf("  Slice1:   %v (len=%d, cap=%d)\n", slice1, len(slice1), cap(slice1))
	fmt.Printf("  Slice2:   %v (len=%d, cap=%d)\n", slice2, len(slice2), cap(slice2))

	// Modify through slice1
	slice1[1] = 99

	fmt.Println("\n  After modifying slice1[1] = 99:")
	fmt.Printf("  Original: %v (affected!)\n", original)
	fmt.Printf("  Slice1:   %v\n", slice1)
	fmt.Printf("  Slice2:   %v (also affected!)\n", slice2)
	fmt.Println("  All slices share the same backing array on heap")
}

// Types
type User struct {
	Name string
	Age  int
}

type LargeObject struct {
	ID   int
	Data []byte
}
