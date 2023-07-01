fn dna_to_rna(dna: &str) -> String {
    // let mut rna: String = String::new();
    // for l in dna.chars() {
    //     if l == 'T' {
    //         rna.push('U');
    //     } else {
    //         rna.push(l);
    //     }
    // }
    // rna
    dna.replace("T", "U")
}

#[cfg(test)]
mod tests {
    use super::dna_to_rna;

    #[test]
    fn returns_expected() {
        assert_eq!(dna_to_rna("TTTT"), "UUUU");
        assert_eq!(dna_to_rna("GCAT"), "GCAU");
    }
}
