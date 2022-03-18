/// Create an empty vector
pub fn create_empty() -> Vec<u8> {
    let v: Vec<u8> = Vec::new(); 
    v
}

/// Create a buffer of `count` zeroes.
///
/// Applications often use buffers when serializing data to send over the network.
pub fn create_buffer(count: usize) -> Vec<u8> {
    (0..count).map(|_| 0).collect()
}

/// Create a vector containing the first five elements of the Fibonacci sequence.
///
/// Fibonacci's sequence is the list of numbers where the next number is a sum of the previous two.
/// Its first five elements are `1, 1, 2, 3, 5`.
pub fn fibonacci() -> Vec<u8> {
    let fibb = vec![1, 1, 2, 3, 5];
    fibb
}
