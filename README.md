## How Little Information Do You Need To Transmit The Entire Wordle List And Its Decoder?

This little project compresses the entire Wordle list plus the decoder into as little space as I could manage.

Many encoding schemes require additional shared data, such as sizeable encoders/decoders with built-in tables, taking up prescious bits on your... magnetic drum? Can't have that.

The requirements for the decoder are as follows:
- Must be able to confirm whether a word is in the list or not.
- Must be able to output a word (in ASCII/UTF8) at a provided index.
- Must be as standalone as possible/reasonable.

My goal was 16KiB (from 12972 words, or 64860 bytes of ASCII/UTF8 without seperation), which I have dubiously achieved 16678 bytes total (~16.2KiB).

How it was done:
- Words were sorted, the first three letters are then removed, and 3 characters are instead used for incrementing the letters at each index progressively.
- This new 'list' is then compressed with a modified LZSS algorithm, generating a seperate list, LZSS matches, again using another character to indicate a match.
- The remaining list is then compressed using Huffman encoding, and a tree is generated for decoding.
- An ELF binary is manually laid out to include headers and the binary data to be packed much more efficiently.
- rustc assembly output of a decoding algorithm is generated and modified, which is all then assembled.
- Voila

Arithmetic coding did give just over 100 bytes of savings over huffman, however I wasn't conviced that it was worth the added complexity to implement it in the decoder.
