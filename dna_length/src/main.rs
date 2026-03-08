fn main() {
    let dna_seq = "ATGCGTAC";
    println!("DNA Sequence Length: {}", dna_seq.len());

    if dna_seq.len() > 10 {
        println!("DNA Sequence is too long");
    } else if dna_seq.len() < 10 {
        println!("DNA Sequence is not long");
    }

}