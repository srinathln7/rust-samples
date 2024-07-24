# Memory Layout

## Resource Acquisition is Initialization (RAII)
The allocation/deallocation strategy we just saw is more generally called RAII, especially in the C++ world. Some prefer the term to Scope-Bound Resource Management (SBRM).RAII/SBRM's key idea is that resources (e.g. memory, file handles, locks, etc) are acquired in a constructor, able to be used for the lifetime/scope of the constructed value, and released in the destructor. Rust's compiler always enforces this behavior for memory. Types, like std::fs::File7 or a user-defined type implementing Drop, can do the same for non-memory resources.

## Size of `Proc` strct

Let's break down how the size of the `Proc` struct is calculated to be 48 bytes on a 64-bit machine. 

### Breakdown of `Proc` struct size

The `Proc` struct has three fields:

1. `name: &'static str`
2. `state: State`
3. `children: Vec<Proc>`

Let's analyze each field:

#### 1. `name: &'static str` (16 bytes)
- On a 64-bit system, a `&str` is a fat pointer consisting of:
  - A pointer to the string data (8 bytes)
  - The length of the string (8 bytes)
- Total: 16 bytes

#### 2. `state: State` (1 byte, padded to 8 bytes)
- The `State` enum is likely represented as a single byte
- However, due to alignment requirements, it's padded to 8 bytes

#### 3. `children: Vec<Proc>` (24 bytes)
- A `Vec<T>` in Rust consists of three components:
  - Pointer to the heap-allocated data (8 bytes)
  - Length of the vector (8 bytes)
  - Capacity of the vector (8 bytes)
- Total: 24 bytes

### Total size calculation

Adding these up:
- `name`: 16 bytes
- `state`: 8 bytes (1 byte + 7 bytes padding)
- `children`: 24 bytes

16 + 8 + 24 = 48 bytes

### Memory layout visualization

Here's a visual representation of the memory layout:

```
[     name (16 bytes)     ][state (8)][    children (24 bytes)    ]
```

### Key points to remember

1. **Alignment**: Rust ensures that fields are aligned properly for efficient memory access. This is why `state` is padded to 8 bytes even though it only needs 1 byte.

2. **Heap vs Stack**: The `Proc` struct itself is 48 bytes, but it may point to additional data on the heap (like the actual contents of the `Vec`).

3. **Fat Pointers**: Both `&str` and `Vec` use fat pointers, which contain extra information beyond just the memory address.

4. **Enum Size**: The `State` enum is small enough to fit in a single byte, but it's padded for alignment.

5. **64-bit Architecture**: This size calculation assumes a 64-bit architecture, where pointers are 8 bytes wide.


## Order of Deallocation

The order in which the names are printed is deterministic because it follows the order of deallocation, which is determined by the ownership hierarchy and the order of the `Vec` elements.

### Ownership Hierarchy

In your `main` function, the ownership hierarchy of the `Proc` instances is as follows:

1. `init` owns `cron` and `rsyslogd`.
2. `rsyslogd` owns `bash`.

When `init` is dropped, it triggers the drop of its children (`cron` and `rsyslogd`). When `rsyslogd` is dropped, it triggers the drop of its child (`bash`).

### Deallocation Order

The deallocation happens in a depth-first manner:

1. `init` is deallocated last because it owns the other processes.
2. `cron` and `rsyslogd` are deallocated before `init` because they are its children.
3. `bash` is deallocated before `rsyslogd` because it is a child of `rsyslogd`.

Therefore, the deallocation order is:

1. `bash`
2. `rsyslogd`
3. `cron`
4. `init`

However, the printed order is reversed because each deallocation prints its message before actually being deallocated. So the print order is:

1. `init`
2. `cron`
3. `rsyslogd`
4. `bash`

## Address Differences

### Stack vs Heap

The first address (`0x7ffe14a25d30`) looks different from the next three (`0x5e12f5cf1bc0`, `0x5e12f5cf1bf0`, `0x5e12f5cf1b80`) because they are allocated in different memory regions:

1. **Stack**: The first address (`0x7ffe14a25d30`) is on the stack. This is where the `init` `Proc` instance is allocated because it is a local variable in the `main` function.
2. **Heap**: The next three addresses (`0x5e12f5cf1bc0`, `0x5e12f5cf1bf0`, `0x5e12f5cf1b80`) are on the heap. These are the addresses of the `Proc` instances (`cron`, `rsyslogd`, `bash`) that are dynamically allocated by the `Vec` inside the `init` `Proc`.

### Summary

- **Order of Deallocation**: Follows a depth-first order due to the ownership hierarchy.
- **Address Differences**: The first address is on the stack, while the others are on the heap.

Understanding these concepts is crucial for managing memory effectively in Rust, especially when dealing with complex data structures and ownership hierarchies.