
extern crate core;

static ARG_COUNT_ERR: &str = "only 1 arg";
static WORD_ARG_ERR: &str = "5 letter words only";
static INDEX_ARG_ERR: &str = "index arg too big";

static EXISTS_SUCCESS: &str = "valid";
static EXISTS_FAILURE: &str = "invalid";

static WORDS_BLOB: &[u8; 13493] =   include_bytes!("../../encoder/words_i3_lzss_he");
static LZSS_MATCHES: &[u8; 2214] = include_bytes!("../../encoder/lzss_matches");


/* static WORDS_BLOB: [u8; 5140] = [2u8; 5140];
static ANS_BITMAP: [u8; 1618] = [2u8; 1618];
static LZSS_MATCHES: [u8; 8520] = [2u8; 8520]; */

static mut LZSS_WINDOW: [u8; 2048] = [b'}'; 2048];
static mut LZSS_WINDOW_CURSOR: usize = 0;

// generated inside the huffman encode function: positive values are indecies pointing to a branch pair, negative are leaf values
static HUFFMAN_TREE: [i8; 58] = [18, 2, 14, 4, -101, 6, 8, -121, -107, 10, 12, -102, -119, -118, // 14
    -125, 16, -116, -126, 28 /* 18 */, 20, -115, 22, 24, -97, 26, -100, -117, -124, 40 /* 28 */, 30, 36, 32, -111, // 33
    34, -112, -99, -114, 38, -109, -103, 44 /* 40 */, 42, -105, -108, -110 /* 44 */, 46, 48, -104, 50, -98, 52, -122, 
    -120, 54, -106, 56, -123, -113];
// vLmkEA==
// b0 21 20
// 11010000

union ArgMem {
    word: [u8; 5],
    num: usize,
}

pub fn main() { unsafe {
    let mut s;
    let mut argv: *mut u8 = core::ptr::NonNull::dangling().as_ptr();
    for arg in std::env::args().enumerate() {
        if arg.0 == 1 {
            s = arg.1;
            s.push(char::from_u32(0).unwrap());
            argv = s.as_mut_ptr();
        }
    }
    
    /* 
    let x = argv.add(1);
    
    let argc: usize;
    core::arch::asm!("pop {}", out(reg) argc, options(preserves_flags));

    if argc != 2 {
        msg_and_exit(ARG_COUNT_ERR as *const _ as *const _, ARG_COUNT_ERR.len());
    } */

    //core::arch::asm!("pop {}", out(reg) argv, options(preserves_flags));

    // argument is null-terminated, thus fin its length, returned length does not include sentinel
    let mut arg_len = len_until(argv, 0);

    // these variables determine the behaviour employed on the decoded data
    let arg_mem;
    let is_index_else_word: bool;

    if *argv >= b'a' {
        if arg_len != 5 { // reject words arguments not 5 letters long
            msg_and_exit(WORD_ARG_ERR.as_ptr(), WORD_ARG_ERR.len());
        }

        arg_mem = ArgMem { word: *(argv as *const _) };
        is_index_else_word = false;
    } else {
        let mut num_val = 0;
        
        while arg_len != 0 {
            arg_len -= 1;
            
            num_val += 10usize.pow(arg_len as u32) * (*argv - b'0') as usize;
            argv = argv.wrapping_add(1);
        }

        if num_val >= 12947 { // reject indecies greater than the length of the wordle list
            msg_and_exit(INDEX_ARG_ERR.as_ptr(), INDEX_ARG_ERR.len());
        }

        arg_mem = ArgMem { num: num_val };
        is_index_else_word = true;
    }
    
    // args parsed; begin decode

    let mut blob_bit_index = 0;
    let mut matches_bit_index = 0;

    let mut zero_index = b'a';
    let mut one_index = b'a';
    let mut two_index = b'h';
    
    let mut word_index = 0;
    let mut char_buf = 0;
    let mut char_await = false;

    loop {
        if word_index == 12947 {
            msg_and_exit(EXISTS_FAILURE.as_ptr(), EXISTS_FAILURE.len());
        }

        // decode huffman
        let mut huffman_index = 0;
        // haffman tree nav loop
        let decode = loop {
            let bit = (*WORDS_BLOB.get_unchecked(blob_bit_index / 8) >> blob_bit_index % 8) & 1;
            blob_bit_index += 1;

            // set bit = use right branch, which is offset by one from the left
            if bit == 1 { huffman_index += 1; }

            let huff = *HUFFMAN_TREE.get_unchecked(huffman_index);
            if huff < 0 { // negative is leaf, positive is branch
                break (-huff) as u8;
            } else {
                huffman_index = huff as usize;
            }
        };
        

        let base = LZSS_WINDOW_CURSOR;
        let mut len = 1;

        if decode == b'~' {
            // lzss

            
            let mut i = 0;
            let mut offset = 0;
            while i < 11 {
                offset |= ((*LZSS_MATCHES.get_unchecked(matches_bit_index / 8) >> matches_bit_index % 8) as usize & 1) << i;

                matches_bit_index += 1;
                i += 1;
            }

            let mut i = 0;
            let mut match_len_bits = 0;
            while i < 3 {
                match_len_bits |= ((*LZSS_MATCHES.get_unchecked(matches_bit_index / 8) >> matches_bit_index % 8) as usize & 1) << i;

                matches_bit_index += 1;
                i += 1;
            }
        
            let mut i = 0;
            while i < match_len_bits + 6 {
                let char = *LZSS_WINDOW.get_unchecked_mut((LZSS_WINDOW_CURSOR + offset) % LZSS_WINDOW.len()); // cursor increments
                *LZSS_WINDOW.get_unchecked_mut(LZSS_WINDOW_CURSOR) = char;
                LZSS_WINDOW_CURSOR = (LZSS_WINDOW_CURSOR + 1) % LZSS_WINDOW.len();

                i += 1;
            }
            
            //base = (LZSS_WINDOW_CURSOR + offset - len) & LZSS_WINDOW.len() - 1;
            len = 6 + match_len_bits;
        } else {
            let window_base = LZSS_WINDOW.get_unchecked_mut(LZSS_WINDOW_CURSOR);
            *window_base = decode;
            LZSS_WINDOW_CURSOR = (LZSS_WINDOW_CURSOR + 1) % LZSS_WINDOW.len();
        }

        let mut i = 0;
        loop {
            let char = *LZSS_WINDOW.get_unchecked((base + i) % LZSS_WINDOW.len());

            if char_await {
                char_await = false;
                word_index += 1;

                /* println!("charbuf: {}", char_buf);
                println!("char: {}", char); */

                /* let mut xxx = String::with_capacity(5);
                xxx.push(char::from_u32(zero_index as u32).unwrap());
                xxx.push(char::from_u32(one_index as u32).unwrap());
                xxx.push(char::from_u32(two_index as u32).unwrap());
                xxx.push(char::from_u32(char_buf as u32).unwrap());
                xxx.push(char::from_u32(char as u32).unwrap());
                println!("{}", xxx); */

                if !is_index_else_word {
                    if arg_mem.word[0] == zero_index 
                    && arg_mem.word[1] == one_index
                    && arg_mem.word[2] == two_index
                    && arg_mem.word[3] == char_buf 
                    && arg_mem.word[4] == char {
                        msg_and_exit(EXISTS_SUCCESS.as_ptr(), EXISTS_SUCCESS.len());
                    }
                } else {
                    if word_index - 1 == arg_mem.num {
                        let word = [zero_index, one_index, two_index, char_buf, char];
                        msg_and_exit(word.as_ptr(), 5);
                    }
                }
            } else if char == b'{' {
                zero_index += 1;
                one_index = b'a';
                two_index = b'a';
            } else if char == b'|' {
                one_index += 1;
                two_index = b'a';
            } else if char == b'}' {
                two_index += 1;
            } else { // character when char buffer is empty - second last letter
                char_await = true;
                char_buf = char;
            }

            i += 1;
            if i < len {
                continue;
            } else {
                break;
            }
        }
    }
}}

pub unsafe fn len_until(ptr: *mut u8, sentinel: u8) -> usize {
    let mut idx = ptr;
    while *idx != sentinel {
        idx = idx.wrapping_add(1);
    }
    idx as usize - ptr as usize
}

#[inline(never)]
pub unsafe fn msg_and_exit(ptr: *const u8, len: usize) -> ! {
    /* core::arch::asm!("
        mov         rdx, {}
        mov         rsi, {}
        mov         rax, 1
        mov         rdi, 1
        syscall
        mov         rax, 60
        mov         rdi, 101
        syscall",
        in(reg) len,
        in(reg) ptr,
        options()
    ); */

        println!("{}", core::str::from_utf8_unchecked(core::slice::from_raw_parts(ptr, len)));

    //loop { }
    panic!()
}
