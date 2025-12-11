


# HTTP server implementation in Rust
#### This project focuses on beginner-friendly raw implementation of http server using Rust.
#### TCP stands for Transmission Control Protocol, a fundamental internet protocol that ensures reliable, ordered, and error-checked delivery of data between applications on a network, working with IP (Internet Protocol) to manage communication by breaking data into packets and reassembling them correctly at the destination.

### Step 1: 

#### Initialize the TCP listener (main.rs)

<img width="808" height="267" alt="Screenshot 2025-12-11 at 6 14 24 PM" src="https://github.com/user-attachments/assets/f4f79f60-b83f-4fae-9ac6-d3b2d29a0d23" />

### Step 2:

#### Parse the raw TCP stream data into HTTP request: (helper.rs)

<img width="1840" height="1326" alt="image" src="https://github.com/user-attachments/assets/81742dd8-632e-4864-ba6e-0d566f3cb8e0" />

### Step 3:

#### Route accordingly as per the parsed HTTP request (main.rs)

<img width="1874" height="1316" alt="image" src="https://github.com/user-attachments/assets/f228ac27-5e32-4669-8658-c0545c259f45" />

### Step 4:

#### To keep it simple, we are using structs in place of database. You can find it in db.rs

<img width="1498" height="1346" alt="image" src="https://github.com/user-attachments/assets/da32a209-d429-40b0-a345-011fe793e4c8" />

### In this tutorial, we are using the uuid crate to generate unique IDs for our tasks.
[uuid] (https://crates.io/crates/uuid/1.19.0)


