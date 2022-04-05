static EXISTS_SUCCESS: &str = "valid";
static EXISTS_FAILURE: &str = "inval";

static WORDS_BLOB: [u8; 13493] = [50u8; 13493];
static LZSS_MATCHES: [u8; 2214] = [50u8; 2214];

static mut LZSS_WINDOW: [u8; 2048] = [b'}'; 2048];
const LZSS_WINDOW_LEN: usize = 2048;

// generated inside the huffman encode function: positive values are indecies pointing to a branch pair, negative are leaf values
static HUFFMAN_TREE: [i8; 58] = [18, 2, 14, 4, -101, 6, 8, -121, -107, 10, 12, -102, -119, -118,
    -125, 16, -116, -126, 28, 20, -115, 22, 24, -97, 26, -100, -117, -124, 40, 30, 36, 32, -111,
    34, -112, -99, -114, 38, -109, -103, 44, 42, -105, -108, -110, 46, 48, -104, 50, -98, 52, -122, 
    -120, 54, -106, 56, -123, -113];

union ArgMem {
    word: [u8; 5],
    num: usize,
}

pub fn main() { unsafe {
    let argv: *const u8;
    
    /*let mut args = std::env::args();
    args.next().unwrap();
    let mut t_arg = args.next().expect("1 arg asseblief");
    t_arg.push(char::from_u32(0).unwrap());
    argv = t_arg.as_ptr();*/
    
    core::arch::asm!("pop {}", out(reg) _, options(preserves_flags));
    core::arch::asm!("pop {}", out(reg) _, options(preserves_flags));
    core::arch::asm!("pop {}", out(reg) argv, options(preserves_flags));

    // cheap and dirty way of checking if the argument is of length 5
    /* if *argv.add(5) != 0 {
        msg_and_exit(ERR.as_ptr(), ERR.len());
    } */

    // these variables determine the behaviour employed on the decoded data
    let arg_mem;
    let is_word_else_index = *argv >= b'a';

    if is_word_else_index {
        arg_mem = ArgMem { word: *(argv as *const _) };
    } else {
        let index = ((*argv - b'0') as usize) << 12
            | ((*argv.wrapping_add(1) - b'0') as usize) << 9
            | ((*argv.wrapping_add(2) - b'0') as usize) << 6
            | ((*argv.wrapping_add(3) - b'0') as usize) << 3
            | ((*argv.wrapping_add(4) - b'0') as usize) << 0;
        arg_mem = ArgMem { num: index };
    }
    
    // args parsed; begin decode

    let mut lzss_window_cursor = 0;
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

        //println!("decode: {}", char::from_u32_unchecked(decode as u32));
        

        let base = lzss_window_cursor;
        let mut len = 1;

        if decode == b'~' {
            // lzss

            let match_index = matches_bit_index / 8;
            let match_bits = (
                ( (*LZSS_MATCHES.get_unchecked(match_index)/* .unwrap() */ as usize)
                | (*LZSS_MATCHES.get_unchecked(match_index + 1)/* .unwrap() */ as usize) << 8
                | (*LZSS_MATCHES.get_unchecked(match_index + 2)/* .unwrap() */ as usize) << 16)
                ) >> (matches_bit_index % 8);
            let offset = match_bits & 0x7FF;
            let match_len_bits = (match_bits & 0x3800) >> 11;
            matches_bit_index += 14;

            /* let mut i = 0;
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
            } */
        
            let mut i = 0;
            while i < match_len_bits + 6 {
                let char = *LZSS_WINDOW.get_unchecked_mut((lzss_window_cursor + offset) % LZSS_WINDOW_LEN); // cursor increments
                *LZSS_WINDOW.get_unchecked_mut(lzss_window_cursor) = char;
                lzss_window_cursor = (lzss_window_cursor + 1) % LZSS_WINDOW_LEN;

                i += 1;
            }
            
            //base = (LZSS_WINDOW_CURSOR + offset - len) & LZSS_WINDOW_LEN - 1;
            len = 6 + match_len_bits;
        } else {
            let window_base = LZSS_WINDOW.get_unchecked_mut(lzss_window_cursor);
            *window_base = decode;
            lzss_window_cursor = (lzss_window_cursor + 1) % LZSS_WINDOW_LEN;
        }

        let mut i = 0;
        loop {
            let char = *LZSS_WINDOW.get_unchecked((base + i) % LZSS_WINDOW_LEN);

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

                if is_word_else_index {
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

#[inline(never)]
pub unsafe fn msg_and_exit(ptr: *const u8, len: usize) -> ! {
    core::arch::asm!("
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
    );

    //println!("{}", core::str::from_utf8_unchecked(core::slice::from_raw_parts(ptr, len)));

    loop { }
}
