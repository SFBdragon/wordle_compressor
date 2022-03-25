
extern crate core;

static ARG_COUNT_ERR: &str = "only 1 arg";
static WORD_ARG_ERR: &str = "5 letter words only";
static INDEX_ARG_ERR: &str = "index arg too big";

static EXISTS_SUCCESS: &str = "valid word";

static WORDS_BLOB: /* & */[u8; 5140] =   [0u8; 5140]; //   include_bytes!("../../encoder/words_i3_lzss_he"); fixme
static LZSS_MATCHES: /* & */[u8; 8520] = [0u8; 8520]; //  include_bytes!("../../encoder/lzss_matches");


/* static WORDS_BLOB: [u8; 5140] = [2u8; 5140];
static ANS_BITMAP: [u8; 1618] = [2u8; 1618];
static LZSS_MATCHES: [u8; 8520] = [2u8; 8520]; */

static mut LZSS_WINDOW: [u8; 1024] = [2u8; 1024];
static mut LZSS_WINDOW_CURSOR: usize = 0;

// generated inside the huffman encode function: positive values are indecies pointing to a branch pair, negative are leaf values
static HUFFMAN_TREE: [i8; 58] = [2, -59, 32, 4, 20, 6, 8, -101, -116, 10, -112, 12, -98, 14, -120, 16, -106, 
    18, -48, -113, 28, 22, -97, 24, 26, -104, -102, -119, 30, -111, -117, -109, 40, 34, 36, -115, 38, -121, 
    -100, -107, 50, 42, 44, -50, -110, 46, -99, 48, -122, -118, 56, 52, -108, 54, -103, -49, -105, -114];


union ArgMem {
    word: [u8; 5],
    num: usize,
}

pub fn main() { unsafe {
    
    let mut argv: *mut u8 = core::ptr::NonNull::dangling().as_ptr();
    for arg in std::env::args().enumerate() {
        if arg.0 == 1 {
            println!("arg.1len {}", arg.1.len());
            let mut str = arg.1;
            str.push(char::from_u32(0).unwrap());
            argv = str.as_mut_ptr();
        }
    }

    let x = argv.add(1);

    /* let argc: usize;
    core::arch::asm!("pop {}", out(reg) argc, options(preserves_flags));

    if argc != 2 {
        msg_and_exit(ARG_COUNT_ERR as *const _ as *const _, ARG_COUNT_ERR.len());
    } */

    //core::arch::asm!("pop {}", out(reg) argv, options(preserves_flags));

    // argument is null-terminated, thus fin its length, returned length does not include sentinel
    let mut arg_len = len_until(argv, 0);

    // these variables determine the behaviour employed on the decoded data
    let mut arg_mem;
    let is_index_else_word: bool;

    if *argv >= b'a' {
        if arg_len != 5 { // reject words arguments not 5 letters long
            msg_and_exit(WORD_ARG_ERR as *const _ as *const _, WORD_ARG_ERR.len());
        }

        arg_mem = ArgMem { word: *(argv as *const _) };
        arg_mem.word[0] -= b'a';
        arg_mem.word[1] -= b'a';
        arg_mem.word[2] -= b'a';
        is_index_else_word = false;
    } else {
        let mut num_val = 0;
        
        loop {
            num_val += 10usize.pow(arg_len as u32) * (*argv - b'0') as usize;
            argv = argv.wrapping_add(1);

            if arg_len == 0 {
                break;
            } else {
                arg_len -= 1;
            }
        }

        if num_val >= 12947 { // reject indecies greater than the length of the wordle list
            msg_and_exit(INDEX_ARG_ERR as *const _ as *const _, INDEX_ARG_ERR.len());
        }

        arg_mem = ArgMem { num: num_val };
        is_index_else_word = true;
    }
    
    // args parsed; begin decode

    let mut blob_bit_index = 0;
    let mut matches_bit_index = 0;

    let mut zero_index = 0;
    let mut one_index = 0;
    let mut two_index = 0;
    
    let mut word_index = 0;
    let mut char_buf = 0;
    let mut char_await = false;

    println!("asdg");

    loop {
        // decode huffman
        let mut huffman_index = 0;
        // haffman tree nav loop
        let decode = loop {
            let bit = (/* * */WORDS_BLOB/* .get_unchecked( */[blob_bit_index / 8]/* ) */ >> blob_bit_index % 8) & 1;
            blob_bit_index += 1;

            // set bit = use right branch, which is offset by one from the left
            if bit == 1 { huffman_index += 1; }

            let huff = /* * */HUFFMAN_TREE[huffman_index]; //.get_unchecked(huffman_index);
            println!("huff {}", huff);
            if huff < 0 { // negative is leaf, positive is branch
                break (-huff) as u8;
            } else {
                huffman_index = huff as usize;
            }
        };

        let mut base = 1023;
        let mut len = 1;

        if decode == b';' {
            // lzss

            let mut i = 0;
            let mut offset = 0;
            while i < 10 {
                offset <<= 1;
                offset |= (*LZSS_MATCHES.get_unchecked(matches_bit_index / 8) >> matches_bit_index % 8) as usize & 1;

                matches_bit_index += 1;
                i -= 1;
            }
            
            let mut i = 0;
            let mut match_len_bits = 0;
            while i < 3 {
                match_len_bits <<= 1;
                match_len_bits |= (*LZSS_MATCHES.get_unchecked(matches_bit_index / 8) >> matches_bit_index % 8) as usize & 1;

                matches_bit_index += 1;
                i -= 1;
            }
        
            let mut i = 0;
            loop {
                let char = *LZSS_WINDOW.get_unchecked_mut(LZSS_WINDOW_CURSOR + offset); // cursor increments
                *LZSS_WINDOW.get_unchecked_mut(LZSS_WINDOW_CURSOR) = char;
                LZSS_WINDOW_CURSOR = (LZSS_WINDOW_CURSOR + 1) % 1024;

                i += 1;
                if i >= len {
                    break;
                }
            }
            
            base = (LZSS_WINDOW_CURSOR + offset - len) & (1024 - 1);
            len = 3 + match_len_bits;
        } else {
            let window_base = LZSS_WINDOW.get_unchecked_mut(LZSS_WINDOW_CURSOR);
            *window_base = decode;
            LZSS_WINDOW_CURSOR = (LZSS_WINDOW_CURSOR + 1) % 1024;
        }

        let mut i = 0;
        loop {
            let char = *LZSS_WINDOW.get_unchecked(base + i);

            if char_await {
                char_await = false;
                word_index += 1;



                let mut xxx = String::with_capacity(5);
                xxx.push(char::from_u32((zero_index + b'a') as u32).unwrap());
                xxx.push(char::from_u32((one_index + b'a') as u32).unwrap());
                xxx.push(char::from_u32((two_index + b'a') as u32).unwrap());
                xxx.push(char::from_u32((char_buf) as u32).unwrap());
                xxx.push(char::from_u32((char) as u32).unwrap());
                println!("{}", xxx);


                

                if is_index_else_word {
                    if arg_mem.word[0] != zero_index { continue; }
                    if arg_mem.word[1] != one_index { continue; }
                    if arg_mem.word[2] != two_index { continue; }
                    if arg_mem.word[3] != char_buf { continue; }
                    if arg_mem.word[4] != char { continue; }
                    
                    msg_and_exit(EXISTS_SUCCESS as *const _ as _, EXISTS_SUCCESS.len());
                } else {
                    if word_index - 1 == arg_mem.num {
                        let word = [zero_index, one_index, two_index, char_buf, char];
                        msg_and_exit(&word as *const _, 5);
                    }
                }
            } else if char == b'0' {
                zero_index += 1;
                one_index = 0;
                two_index = 0;
            } else if char == b'1' {
                one_index += 1;
                two_index = 0;
            } else if char == b'2' {
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

    let mut x = String::with_capacity(len);
    for i in 0..len {
        x.push(char::from_u32_unchecked((*ptr.offset(i as isize)) as u32));
    }

    print!("{}", x);

    //loop { }
    panic!()
}
