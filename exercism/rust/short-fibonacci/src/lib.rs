/// Create an empty vector
pub fn create_empty() -> Vec<u8> {
    let v: Vec<u8> = Vec::new(); 
    v
}

/// Create a buffer of `count` zeroes.
///
/// Applications often use buffers when serializing data to send over the network.
pub fn create_buffer(count: usize) -> Vec<u8> {
    // unimplemented!("create a zeroized buffer of {} bytes", count)
    let mut buffer = create_empty();
    
    for _ in 0..count {
        buffer.push(0);
    }

    buffer
}

/// Create a vector containing the first five elements of the Fibonacci sequence.
///
/// Fibonacci's sequence is the list of numbers where the next number is a sum of the previous two.
/// Its first five elements are `1, 1, 2, 3, 5`.
pub fn fibonacci() -> Vec<u8> {
    let fibb = vec![1, 1, 2, 3, 5];
    fibb
}