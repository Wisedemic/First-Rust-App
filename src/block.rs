// Define what a block is
#[derive(Debug)]
pub struct Block {
	data: String
}

// Implement some functions for the Block struct
impl Block  {
	// Create a block constructor
	pub fn new(data: String) -> Block {
		Block { data }
	}
}
