use std::fs::File;
use std::error::Error;
use std::collections::HashMap;
use std::io::prelude::*;

pub fn compress(file_path: &str) -> Result<(), Box<dyn Error>> {
    let mut xml_file = File::open(file_path).expect("Error opening file");
    let mut buffer = vec![];
    let mut bytepair_map: HashMap<(u8, u8), u64> = HashMap::new();
    let mut translation_table: HashMap<u8, (u8, u8)> = HashMap::new();
    let mut new_buffer = vec![]; 

    let read_bytes = xml_file.read_to_end(&mut buffer)?;
    
    // yikes
    let mut cloned_buffer = buffer.clone();
    cloned_buffer.sort();
    cloned_buffer.dedup();
    let num_distinct = cloned_buffer.len();
    cloned_buffer.clear();

    loop {
        let rand_byte: u8 = rand::random();

        for pair in buffer.iter().zip(buffer.iter().skip(1)) {
            // I don't know if this is the best to do it
            *bytepair_map.entry((*pair.0, *pair.1)).or_insert(1) += 1;
        }
        let max_pair= bytepair_map.iter()
            .reduce(|current_max, (byte_pair, freq)| {
                if freq > current_max.1 {
                    (byte_pair, freq)
                } else {
                    current_max
                }
            }).unwrap().0;

        if buffer.contains(&rand_byte) {
            continue;
        }

        translation_table.insert(rand_byte, *max_pair);
       
        // This probably needs a more rusty way to do it 
        let mut skip_next_iter = false;
        for i in 0..(buffer.len() - 1) {
            if skip_next_iter {
                skip_next_iter = false;
                continue;
            }

            if buffer[i] == max_pair.0 && buffer[i+1] == max_pair.1 {
                new_buffer.push(rand_byte);
                skip_next_iter = true;
                continue;
            }
            new_buffer.push(buffer[i]);
        }

        buffer = new_buffer.clone();
        new_buffer.clear();
        bytepair_map.clear();
        if translation_table.len() == 256 - num_distinct {
            break;
        }
    }
    
    for (change, (b1, b2)) in translation_table.iter() {
        buffer.push(*change);
        buffer.push(*b1);
        buffer.push(*b2);
    }

    let mut compressed = File::create("compressed.xip")?;
    let written_bytes = compressed.write(&buffer)?;
    println!("{:?}", translation_table);
    println!("Old buffer size: {}", read_bytes);
    println!("buffer size after first iter{}", written_bytes);

    Ok(())
}

pub fn decompress(file_path: &str) {
    unimplemented!();
}

