fn main() {
let dna_seq = "ATGCGTAC";

let start = &dna_seq[0..3];
let middle = &dna_seq[3..5];
let end = &dna_seq[6..8];

println!("Start codon: {} \nMiddle codon: {} \nEnd codon: {}", start, middle, end);


}