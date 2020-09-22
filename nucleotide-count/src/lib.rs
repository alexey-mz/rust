use std::collections::HashMap;

pub fn count(nucleotide: char, dna: &str) -> Result<usize, char> {
    let mut res = 0;
    match nucleotide {
        'A' | 'C' | 'G' | 'T' => {
            for ch in dna.chars() {
                match ch {
                    'A' | 'C' | 'G' | 'T' => if ch == nucleotide {
                        res += 1;
                    },
                    _ => {
                        return Err(ch)
                    }
                }
            }
        },
        _ => return Err(nucleotide)
    }

    Ok(res)
}

pub fn nucleotide_counts(dna: &str) -> Result<HashMap<char, usize>, char> {
    let mut nucleotides = HashMap::new();
    nucleotides.insert('A', 0);
    nucleotides.insert('C', 0);
    nucleotides.insert('G', 0);
    nucleotides.insert('T', 0);
    for ch in dna.chars() {
        match ch {
            'A' | 'C' | 'G' | 'T' => {
                let t = nucleotides.get_mut(&ch).unwrap();
                *t += 1;
            },
            _ => return Err(ch)
        }
    }
    Ok(nucleotides)
}
