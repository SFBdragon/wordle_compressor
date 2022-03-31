//#![feature(seek_stream_len)]

use std::{path::Path, io::{Read, Write, Seek}, collections::{HashMap, VecDeque}, panic, ffi::OsStr, fs::File};

/*
Notes:
0..25 : letters
25..32 : indecies 0, 1, 2, 2_2, 2_3, 2_4, 2_5
// scrap 2_3 and 2_5? retry huffman for each

srtd_words (without whitespace): 64735B
alphabetical indexed 2 levels: 39484B
alphabetical indexed 3 levels: 32447B

i3 w/counted indecies 29375B  (w/5bits: 18360B) (w/deflate: 14768B)
i3 w/ 1 counted indecies w/ huffman encoding 16312B
i3 w/ 2 counted indecies w/ huffman encoding 16162B

LZSS


radix/other trie compression

*/

fn main() {
    println!("Hello, world!");

    for file in ["srtd_words", "words_i3", "words_i3_lzss", "words_i3_lzss_he", "words_i3_lzss_ari", "lzss_matches"] { }
    
    //create_answers_bitmap();
    
    index_count_srtd_words_3();
    //radix_trie_encode();
    lzss_sep_matches(&Path::new("words_i3"));
    huffman_encode(&Path::new("words_i3_lzss"));
    arithmetic_compression(&Path::new("words_i3_lzss"));
    //huffman_encode();
    arithmetic_compression(&Path::new("words_i3"));
    /* ;
     */

    /* let mut srtd_words_fhandle = std::fs::File::open(Path::new("srtd_words")).unwrap();
    let mut srtd_buf = Vec::new();
    srtd_words_fhan
        /* let mut fhandle = File::open(Path::new(file)).unwrap();
        let mut buf = Vec::new();
        fhandle.read_to_end(&mut buf).unwrap();

        let deflate = deflate::deflate_bytes_conf(&buf[..], deflate::CompressionOptions::high());
        println!("deflate: {}: {}", file, deflate.len()); */

        /* let mut out_buf = Vec::new();
        let brotli = brotli::BrotliCompress(&mut fhandle, &mut out_buf, &brotli::enc::BrotliEncoderParams {
            dist: todo!(),
            mode: brotli::enc::encode::Brotly,
            quality: 10,
            q9_5: todo!(),
            lgwin: todo!(),
            lgblock: todo!(),
            size_hint: todo!(),
            disable_literal_context_modeling: todo!(),
            hasher: brotli::enc::backward_references::BrotliHasherParams {
                type_: todo!(),
                bucket_bits: todo!(),
                block_bits: todo!(),
                hash_len: todo!(),
                num_last_distances_to_check: todo!(),
                literal_byte_score: todo!(),
            },
            log_meta_block: false,
            stride_detection_quality: 10,
            high_entropy_detection_quality: true,
            cdf_adaptation_detection: todo!(),
            prior_bitmask_detection: 0,
            literal_adaptation: Default::default(),
            large_window: false,
            avoid_distance_prefix_search: false,
            catable: false,
            use_dictionary: true,
            appendable: false,
            favor_cpu_efficiency: false,
        }).unwrap();
        println!("brotli: {}: {}, {}", file, out_buf.len(), brotli); */
        dle.read_to_end(&mut srtd_buf).unwrap();

    let mut words_i3_fhandle = std::fs::File::open(Path::new("words_i3")).unwrap();
    let mut i3_buf = Vec::new();
    words_i3_fhandle.read_to_end(&mut i3_buf).unwrap();

    println!("deflate of srtd_words len: {}", deflate::deflate_bytes(&srtd_buf).len());
    println!("deflate of words_i3 len: {}", deflate::deflate_bytes(&i3_buf).len());
 */
    //index_count_srtd_words_2();

    // sort_words();
}


fn radix_trie_encode() {
    // aahed aalii aargh aarti abaca abaci abbab
    // aahedaaliiaarghaartiabacaabaciabbab
    // aahed}lii}rghti}}bacaci}bab
    // }}}}}}}ed}}}}ii}}}}}}ghti|caci}ab
    // 2 
    // a0a1hed1lii1r2gh2ti1ba3c4a4i
    // a{a{hedliirghrti2{ac{ai
    // aah|edl|iirghti|+a|c{ai
    // aah}edl}iirghti}+ac{ai}+ab
    // a0a1h2|1e1d[]1l2|1i1i[]1r2g2h[]2t2i[]2|1+(ab)a2c2{3a[]3i[]3|2+(abb)2ab[]2
    // abaca+
    // ddd  
    // increment behind the level, increment does not drop lvl into a nonassume dlevel
    
    // aahedliirghrti2ac{ai

    // auto drop down after an entry at an auto drop index
    // if |, hike the lvl
    // if invalid a seq, hike the lvl
    // if +, inc at the level, drop a level if auto drop index
    // if invalid z seq, drop a level
    
    
    // bemad bemas bemix bemud bench bends bendy benes benet benga
    // bemadbemasbemixbemudbenchbendsbendybenesbenetbenga
    // bemadasixud}nchdsdyesetga
    // bema{ds}ixud+chd{sy}e{st}ga
    // bemadasixud+chdsdyesetga
    // bemadasixud3chdsdyesetga
    // be{m{adasixud3{chdsdyesetga
    // bemadasixud+chdsdyesetga
    // be{m{a{ds}ixud}n{chd{sy}e{st}ga
    
    // word endings are implicit and hence transition needn't be marked
    // levelu/leveld markers or nleveld nlevelu markers?
    // increment using a index-dedicated symbol or i-ded after levelu
    // on level down, indicate, on level up, indicate unless not necessary
    // // not necessary at EOF
    // 
    // on the levels where it is more common to remain, indicate deltas, else indicate non-deltas
    // leveld L, levelu l, delta D, ndelta d
    
    
    
    /* const WORD_LENGTH: usize = 5;
    const ASSUME_LEVELD_INDEX: u8 = 3;
    
    const LEVELD: u8 = b'{';
    const LEVELU: u8 = b'}';
    const NLEVELD: u8 = b'|';
    const INC: u8 = b'+'; */
    


    let mut srtd_words_fhandle = std::fs::File::open(Path::new("srtd_words")).expect("srtd_words file open failed");
    let mut words_char_list = Vec::new();
    srtd_words_fhandle.read_to_end(&mut words_char_list).expect("failed to read words");

    let words_string = String::from_utf8(words_char_list).unwrap();
    let mut words_vec = words_string.split('\n').collect::<Vec<_>>();
    for i in 0..words_vec.len() { words_vec[i] = words_vec[i].trim(); }

    let mut words_i3_fhandle = std::fs::File::create(Path::new("words_i3")).unwrap();

    let mut last = [b'a', b'a', b'h',/*  b'e' */];

    'next: for w in words_vec {
        let w = w.as_bytes();
        for i in 0..3 {
            if w[i] != last[i] {
                for j in i..3 {
                    last[j] = w[j];
                    words_i3_fhandle.write_all(&[b'}']).unwrap();
                }
                words_i3_fhandle.write_all(&w[i..5]).unwrap();
                continue 'next;
            }
        }
        words_i3_fhandle.write_all(&w[3..5]).unwrap();
    }
}


const ZERO_INDEX_CHAR: u8 = b'{';
const ONES_INDEX_CHAR: u8 = b'|';
const TWOS_INDEX_CHAR: u8 = b'}';
const LZSS_MATCH_CHAR: u8 = b'~';
const BASE_RANGE_CHAR: u8 = b'a';
const ACME_RANGE_CHAR: u8 = LZSS_MATCH_CHAR;
const CHAR_RANGE_LEN: usize = (ACME_RANGE_CHAR - BASE_RANGE_CHAR + 1) as usize;


#[allow(dead_code)]
fn arithmetic_compression(src: impl AsRef<Path>) {
    let mut src_fhandle = File::open(src.as_ref()).unwrap();
    let mut src_buf = Vec::with_capacity(10000/* src_fhandle.stream_len().unwrap() as usize */);
    src_fhandle.read_to_end(&mut src_buf).unwrap();

    // GET SYMBOL FREQUENCIES
    
    let mut char_freqs = [0u64; 256 + 1];
    // count frequencies
    for &b in src_buf.iter() { char_freqs[(b + 1) as usize] += 1; }
    // convert to cumulative frequencies
    for i in 1..char_freqs.len() { char_freqs[i] += char_freqs[i - 1]; }
    // store total frequency in a register
    let total_freq = char_freqs[char_freqs.len() - 1];

    // DECODE

    let mut output = Vec::with_capacity(src_buf.len()); // output should hopefully be shorter than input
    for _ in 0..src_buf.len() { output.push(0u8); }
    let mut output_bit_index = 0;
    
    let mut lo = 0u64;
    let mut hi = u32::MAX as u64;
    
    let mut underflow_bits = 0;
    // 0, 3, 7, 9


    for &b in src_buf.iter() {
        let range = (hi - lo + 1) as u64;
        hi = lo + range * char_freqs[(b+1) as usize] / total_freq - 1;
        lo = lo + range * char_freqs[b as usize] / total_freq;

        assert!(hi < 1 << 32, "{:#x}, {:#x}", lo, hi);
        assert!(lo < 1 << 32, "{:#x}, {:#x}", lo, hi);

        loop {
            if lo & (1 << 31) != 0 { // hi & lo's hi bit is 1
                output[output_bit_index / 8] |= 1 << (output_bit_index % 8);
                output_bit_index += 1 + underflow_bits;
                underflow_bits = 0;
            } else if hi & (1 << 31) == 0 { // hi & lo's hi bit is 0
                output_bit_index += 1;
                for _ in 0..underflow_bits {
                    output[output_bit_index / 8] |= 1 << (output_bit_index % 8);
                    output_bit_index += 1;
                }
                underflow_bits = 0;
            } else if lo & (1 << 30) != 0 && hi & (1 << 30) == 0 { // underflow condition
                underflow_bits += 1;
                lo -= 1 << 30;
                hi -= 1 << 30;
            } else {
                break;
            }

            lo = lo << 1 & (1 << 32) - 1;
            hi = hi << 1 & (1 << 32) - 1 | 1;
        }
    }

    if lo & 1 << 30 == 0 {
        for _ in 0..underflow_bits {
            output[output_bit_index / 8] |= 1 << (output_bit_index % 8);
            output_bit_index += 1;
        }
    }
    output[output_bit_index / 8] |= 1 << (output_bit_index % 8);

    println!("arithmetic coded bytes: {}", (output_bit_index + 7) / 8);
    
    let mut file_name = src.as_ref().file_name().unwrap().to_owned();
    file_name.push(OsStr::new("_ari"));
    let mut ari_fhandle = File::create(src.as_ref().with_file_name(&file_name).as_path()).unwrap();
    ari_fhandle.write_all(&output[0..((output_bit_index + 7) / 8)]).unwrap();

}


#[allow(dead_code)]
fn create_answers_bitmap() {
    let mut srtd_words_fhandle = std::fs::File::open(Path::new("srtd_words")).unwrap();
    let mut words_char_list = Vec::new();
    srtd_words_fhandle.read_to_end(&mut words_char_list).unwrap();

    let words_string = String::from_utf8(words_char_list).unwrap();
    let mut words_vec = words_string.split('\n').collect::<Vec<_>>();
    for i in 0..words_vec.len() { words_vec[i] = words_vec[i].trim(); }

    
    let mut answer_words_fhandle = std::fs::File::open(Path::new("answers")).unwrap();
    let mut words_char_list = Vec::new();
    answer_words_fhandle.read_to_end(&mut words_char_list).unwrap();

    let answers_string = String::from_utf8(words_char_list).unwrap();
    let mut answers_vec = answers_string.split('\n').collect::<Vec<_>>();
    for i in 0..answers_vec.len() { answers_vec[i] = answers_vec[i].trim(); }

    let mut answer_bitmap_fhandle = std::fs::File::create(Path::new("answer_words_bitmap")).unwrap();

    let mut longest_nul_str = 0usize;
    let mut curr_nul_str = 0usize;

    for i in 0..(words_vec.len() / 8) {
        let mut byte = 0;
        for j in 0..8.min(words_vec.len() - i * 8) {
            let word = words_vec[i * 8 + j];
            if answers_vec.iter().any(|x| *x == word) {
                byte |= 1 << j;

                if curr_nul_str > longest_nul_str {
                    longest_nul_str = curr_nul_str;
                }
                curr_nul_str = 0;
            } else {
                curr_nul_str += 1;
            }
        }
        answer_bitmap_fhandle.write_all(&[byte]).unwrap();
    }

    println!("{}", longest_nul_str);
}

#[allow(dead_code)]
fn lzss_sep_matches(src: impl AsRef<Path>) {
    let mut src_fhandle = File::open(src.as_ref()).unwrap();
    let mut src_buf = Vec::with_capacity(10000/* src_fhandle.stream_len().unwrap() as usize */);
    src_fhandle.read_to_end(&mut src_buf).unwrap();
    src_buf.reverse();

    const MATCH_BITS: usize = 3;
    const WINDOW_BITS: usize = 11;

    const MATCH_MIN: usize = 6;
    const MATCH_MAX: usize = MATCH_MIN + (1 << MATCH_BITS) - 1;
    const SEARCH_WINDOW_LEN: usize = 1 << WINDOW_BITS;

    let mut window = VecDeque::with_capacity(SEARCH_WINDOW_LEN);
    let mut buffer = VecDeque::with_capacity(MATCH_MAX);

    for _ in 0..SEARCH_WINDOW_LEN { window.push_back(TWOS_INDEX_CHAR); }
    for _ in 0..src_buf.len().min(MATCH_MAX) { buffer.push_back(src_buf.pop().unwrap()); }

    let mut matches_vec = Vec::new();
    let mut encoded_bytes = Vec::new();

    loop {
        let mut best_match = (0, 0);

        for i in 0..window.len() {
            if window[i] == buffer[0] {
                let mut len = 0;
                for j in 0..(buffer.len().min(window.len() - i)) {
                    if window[i + j] != buffer[j] {
                        len = j;
                        break;
                    }
                }

                if len > best_match.1 { best_match = (i, len); }
            }
        }

        if best_match.1 >= MATCH_MIN {
            for _ in 0..best_match.1 {
                window.pop_front().unwrap();
                window.push_back(buffer.pop_front().unwrap());
                if let Some(bb) = src_buf.pop() {
                    buffer.push_back(bb);
                }
            }

            encoded_bytes.push(LZSS_MATCH_CHAR);
            matches_vec.push((best_match.0, best_match.1 - MATCH_MIN));
        } else {
            let encode = buffer.pop_front().unwrap();
            encoded_bytes.push(encode);
            window.pop_front().unwrap();
            window.push_back(encode);
            if let Some(bb) = src_buf.pop() {
                buffer.push_back(bb);
            }
        }

        if buffer.is_empty() && src_buf.is_empty() { break; }
    }

    println!("encoded bytes len: {}", encoded_bytes.len());
    println!("matches list bytes: {}", (matches_vec.len() * (WINDOW_BITS + MATCH_BITS) + 7) / 8);

    let mut file_name = src.as_ref().file_name().unwrap().to_owned();
    file_name.push(OsStr::new("_lzss"));
    let mut lzss_fhandle = File::create(src.as_ref().with_file_name(&file_name).as_path()).unwrap();
    lzss_fhandle.write_all(&encoded_bytes).unwrap();

    let mut lzss_matches_fhandle = std::fs::File::create(Path::new("lzss_matches")).unwrap();

    println!("{:?}", matches_vec[0]);
    println!("{:?}", matches_vec[1]);
    println!("{:?}", matches_vec[2]);

    // 00 08 00 30 77
    // 0000 0000 0001 0000 0000 0000 0000 1100 1110 1110
    // 0                1                 2             
    
    let mut buffer = 0usize;
    let mut buf_len = 0usize;
    for (offset, len) in matches_vec {
        buffer |= (offset | (len << WINDOW_BITS)) << buf_len;
        buf_len += MATCH_BITS + WINDOW_BITS;

        while buf_len >= 8 {
            lzss_matches_fhandle.write_all(&[buffer as u8]).unwrap();
            buf_len -= 8;
            buffer >>= 8;
        }
    }
    
    if buf_len > 0 {
        lzss_matches_fhandle.write_all(&[buffer as u8]).unwrap();
    }
}

#[allow(dead_code)]
fn lzss(src: impl AsRef<Path>) {
    let mut src_fhandle = File::open(src.as_ref()).unwrap();
    let mut src_buf = Vec::with_capacity(10000/* src_fhandle.stream_len().unwrap() as usize */);
    src_fhandle.read_to_end(&mut src_buf).unwrap();
    src_buf.reverse();

    const MATCH_BITS: usize = 3;
    const WINDOW_BITS: usize = 10;

    const MATCH_MIN: usize = 4;
    const MATCH_MAX: usize = MATCH_MIN + (1 << MATCH_BITS) - 1;
    const SEARCH_WINDOW_LEN: usize = 1 << WINDOW_BITS;

    let mut window = VecDeque::with_capacity(SEARCH_WINDOW_LEN);
    let mut buffer = VecDeque::with_capacity(MATCH_MAX);

    for _ in 0..SEARCH_WINDOW_LEN { window.push_back(b'2'); }
    for _ in 0..src_buf.len().min(MATCH_MAX) { buffer.push_back(src_buf.pop().unwrap()); }

    let mut matches_vec = Vec::new();
    let mut encoded_bytes = Vec::new();

    loop {
        let mut best_match = (0, 0); // offset, size

        for i in 0..window.len() {
            if window[i] == buffer[0] {
                let mut len = 0;
                for j in 0..(buffer.len().min(window.len() - i)) {
                    if window[i + j] != buffer[j] {
                        len = j;
                        break;
                    }
                }

                if len > best_match.1 { best_match = (i, len); }
            }
        }

        if best_match.1 >= MATCH_MIN {
            for _ in 0..best_match.1 {
                window.pop_front().unwrap();
                window.push_back(buffer.pop_front().unwrap());
                if let Some(bb) = src_buf.pop() {
                    buffer.push_back(bb);
                }
            }

            /* let match_u16 = 1 << 15 | (best_match.1 as u16) << WINDOW_BITS | best_match.0 as u16;
            encoded_bytes.push(match_u16 as u8);
            encoded_bytes.push((match_u16 >> 8) as u8); */
            encoded_bytes.push(LZSS_MATCH_CHAR);

            matches_vec.push(best_match);
        } else {
            let encode = buffer.pop_front().unwrap();
            encoded_bytes.push(encode);

            window.pop_front().unwrap();
            window.push_back(encode);
            if let Some(bb) = src_buf.pop() {
                buffer.push_back(bb);
            }
        }

        if buffer.is_empty() && src_buf.is_empty() { break; }
    }

    println!("encoded bytes len: {}", encoded_bytes.len());
    println!("matches list bytes: {}", (matches_vec.len() * (WINDOW_BITS + MATCH_BITS) + 7) / 8);

    let mut file_name = src.as_ref().file_name().unwrap().to_owned();
    file_name.push(OsStr::new("_lzss"));
    let mut lzss_fhandle = File::create(src.as_ref().with_file_name(&file_name).as_path()).unwrap();

    lzss_fhandle.write_all(&encoded_bytes).unwrap();

    let mut lzss_matches_fhandle = std::fs::File::create(Path::new("lzss_matches")).unwrap();
    
    let mut buffer = 0usize;
    let mut buf_len = 0usize;
    for (offset, len) in matches_vec {
        buf_len += MATCH_BITS + WINDOW_BITS;
        buffer <<= MATCH_BITS + WINDOW_BITS;
        buffer |= len | (offset << MATCH_BITS);

        while buf_len >= 8 {
            lzss_matches_fhandle.write_all(&[(buffer >> buf_len - 8) as u8]).unwrap();
            buf_len -= 8;
            buffer >>= 8;
        }
    }
    
    if buf_len > 0 {
        lzss_matches_fhandle.write_all(&[(buffer << 8 - buf_len) as u8]).unwrap();
    }
}



#[derive(Debug, Clone, Hash, PartialEq, Eq)]
enum Node {
    Branch((Box<Node>, Box<Node>)),
    Leaf(u8),
}

#[allow(dead_code)]
fn huffman_encode(src: impl AsRef<Path>) {
    let mut src_fhandle = File::open(src.as_ref()).unwrap();
    let mut src_buf = Vec::with_capacity(10000/* src_fhandle.stream_len().unwrap() as usize */);
    src_fhandle.read_to_end(&mut src_buf).unwrap();

    let mut syms: HashMap<_, usize> = HashMap::new();
    
    for &b in src_buf.iter() {
        if let Some(&count) = syms.get(&Node::Leaf(b)) {
            syms.insert(Node::Leaf(b), count + 1);
        } else {
            syms.insert(Node::Leaf(b), 1);
        }
    }

    let mut pairs = syms.into_iter().collect::<Vec<_>>();
    pairs.sort_by_key(|p| usize::MAX - p.1);

    while pairs.len() > 1 {
        let last = pairs.pop().unwrap();
        let second_last = pairs.pop().unwrap();

        let composite = Node::Branch((Box::new(second_last.0), Box::new(last.0)));
        let frequency = second_last.1 + last.1;
        let index = pairs.binary_search_by_key(&(usize::MAX - frequency), |p| usize::MAX - p.1).unwrap_or_else(|e| e);

        pairs.insert(index, (composite, frequency));
    }

    assert_eq!(pairs.len(), 1);
    let root = pairs.pop().unwrap();

    let mut table = HashMap::<u8, _>::new();
    let mut tree_nav_stack = vec![(&root.0, 0u64, 0usize)]; // node, partial huffman code, depth

    while let Some(node) = tree_nav_stack.pop() {
        match node.0 {
            Node::Branch((a, b)) => {
                tree_nav_stack.push((a, node.1, node.2 + 1));
                tree_nav_stack.push((b, node.1 | 1 << node.2, node.2 + 1));
            },
            Node::Leaf(val) =>  { 
                assert!(table.insert(*val, (node.1, node.2)).is_none());
            },
        }
    }

    // PRINT TREE AS ARRAY

    println!("{:?}", root.0);

    /* Branch((
        Branch((
            Branch((
                Branch((
                    Branch((
                        Leaf(110),
                        Branch((
                            Branch((
                                Branch((
                                    Branch((
                                        Leaf(120),
                                        Branch((
                                            Leaf(106),
                                            Branch((
                                                Leaf(123), 
                                                Leaf(113)
                                            ))
                                        ))
                                    )),
                                    Leaf(122)
                                )), 
                                Leaf(98)
                            )), 
                            Leaf(104)
                        ))
                    )), 
                    Branch((
                        Leaf(105), 
                        Leaf(108)
                    ))
                )), 
                Branch((
                    Branch((
                        Leaf(114), 
                        Branch((
                            Leaf(109), 
                            Leaf(103)
                        ))
                    )), 
                    Branch((
                        Leaf(111), 
                        Branch((
                            Leaf(112), 
                            Leaf(99)
                        ))
                    ))
                ))
            )), 
            Branch((
                Leaf(115), 
                Branch((
                    Branch((
                        Branch((
                            Leaf(117), 
                            Leaf(124)
                        )), 
                        Leaf(100)
                    )), 
                    Leaf(97)
                ))
            ))
        )), 
        Branch((
            Branch((
                Leaf(125), 
                Branch((
                    Leaf(116), 
                    Leaf(126)
                ))
            )), 
            Branch((
                Leaf(101), 
                Branch((
                    Branch((
                        Leaf(107), 
                        Branch((
                            Branch((
                                Leaf(119), 
                                Leaf(118)
                            )), 
                            Leaf(102)
                        ))
                    )), 
                    Leaf(121)
                ))
            ))
        ))
    )) */

    let mut table_bytes = Vec::new();
    let mut tree_rev_nav_map = HashMap::new(); // code: offset
    tree_nav_stack.push((&root.0, 0, 0)); // node, code, nada
    while let Some(node) = tree_nav_stack.pop() {

        match node.0 {
            Node::Branch((a, b)) => {
                if let Some(&offset) = tree_rev_nav_map.get(&node.1) {
                    table_bytes[offset] = table_bytes.len() as i8;
                }

                tree_nav_stack.push((a, node.1 << 1, node.2 + 1));
                tree_rev_nav_map.insert(node.1 << 1, table_bytes.len());
                table_bytes.push(0i8);
                tree_nav_stack.push((b, node.1 << 1 | 1, node.2 + 1));
                tree_rev_nav_map.insert(node.1 << 1 | 1, table_bytes.len());
                table_bytes.push(0i8);
            },
            Node::Leaf(val) => {
                if let Some(&offset) = tree_rev_nav_map.get(&node.1) {
                    table_bytes[offset] = -(*val as i8);
                } else {
                    panic!();
                }
            },
        }
    }

    println!("{}", table_bytes.len());
    for i in table_bytes {
        print!("{}, ", i);
    }
    println!();


    // ENCODE BYTES

    let mut i3_he_fhandle = std::fs::File::create(Path::new("words_i3_lzss_he")).unwrap();

    let mut total_len = 0usize;
    
    let mut buffer = 0u64;
    let mut buf_len = 0usize;
    for &b in src_buf.iter() {
        let huff_code = table[&b];
        total_len += huff_code.1;
        assert!(buf_len + huff_code.1 < 64);

        buffer |= huff_code.0 << buf_len;
        buf_len += huff_code.1;

        while buf_len >= 8 {
            i3_he_fhandle.write_all(&[buffer as u8]).unwrap();
            buf_len -= 8;
            buffer >>= 8;
        }
    }

    // 01 110 011
    // 01110
    // 01 11 
    
    if buf_len > 0 {
        i3_he_fhandle.write_all(&[buffer as u8]).unwrap();
        println!("bit offset from EOF: {}", 8 - buf_len);
        total_len += 8 - buf_len;
    }
    
    println!("total huffman encode len: {}", total_len / 8);
}


#[allow(dead_code)]
fn index_count_srtd_words_3() {
    let mut srtd_words_fhandle = std::fs::File::open(Path::new("srtd_words")).expect("srtd_words file open failed");
    let mut words_char_list = Vec::new();
    srtd_words_fhandle.read_to_end(&mut words_char_list).expect("failed to read words");

    let words_string = String::from_utf8(words_char_list).unwrap();
    let mut words_vec = words_string.split('\n').collect::<Vec<_>>();
    for i in 0..words_vec.len() { words_vec[i] = words_vec[i].trim(); }

    let mut words_i3_fhandle = std::fs::File::create(Path::new("words_i3")).unwrap();

    let mut counters = [b'a', b'a', b'h'];

    for w in words_vec {
        for i in 0..3 {
            for _ in 0..(w.as_bytes()[i] - counters[i]) {
                words_i3_fhandle.write_all(&[i as u8 + ZERO_INDEX_CHAR]).unwrap();
                counters[i] += 1;
                
                for c in &mut counters[(i + 1)..3] {
                    *c = b'a';
                }
            }
        }
        
        words_i3_fhandle.write_all(&w.as_bytes()[3..5]).unwrap();
    }
}

#[allow(dead_code)]
fn sort_words() {
    let mut words_fhandle = std::fs::File::open(Path::new("words")).expect("words file open failed");
    let mut words_char_list = Vec::new();
    words_fhandle.read_to_end(&mut words_char_list).expect("failed to read words");

    let words_string = String::from_utf8(words_char_list).unwrap();
    let mut words_vec = words_string.split('\n').collect::<Vec<_>>();

    for i in 0..words_vec.len() {
        words_vec[i] = words_vec[i].trim();
    }

    words_vec.sort();

    let mut srtd_words_fhandle = std::fs::File::create(Path::new("srtd_words")).expect("create srtd_words failed");
    
    for i in 0..words_vec.len() {
        srtd_words_fhandle.write_all(words_vec[i].as_bytes()).unwrap();
        srtd_words_fhandle.write_all(&[b'\n']).unwrap();
    }
}
