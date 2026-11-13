//! Internal SHA-256, used only to anchor the integrity of the normative fixture.
//!
//! `01` §6.4: the digest has to be recomputable inside the crate, without a
//! dependency, so the core can prove its own fixture integrity. An internal
//! implementation is not a dependency; it is forty lines of integer arithmetic.
//!
//! The arithmetic here is *modular*: SHA-256 is defined over `u32` lanes that wrap
//! by construction, so `wrapping_add` is the correct operation and not an escape
//! from the checked discipline of `01` §4.3, which governs the fixed point
//! quantities. The only quantity crossed with the outside world is the bit length,
//! which is `u64` and is encoded with `to_be_bytes` rather than a cast.

const K: [u32; 64] = [
    0x428a2f98, 0x71374491, 0xb5c0fbcf, 0xe9b5dba5, 0x3956c25b, 0x59f111f1, 0x923f82a4, 0xab1c5ed5,
    0xd807aa98, 0x12835b01, 0x243185be, 0x550c7dc3, 0x72be5d74, 0x80deb1fe, 0x9bdc06a7, 0xc19bf174,
    0xe49b69c1, 0xefbe4786, 0x0fc19dc6, 0x240ca1cc, 0x2de92c6f, 0x4a7484aa, 0x5cb0a9dc, 0x76f988da,
    0x983e5152, 0xa831c66d, 0xb00327c8, 0xbf597fc7, 0xc6e00bf3, 0xd5a79147, 0x06ca6351, 0x14292967,
    0x27b70a85, 0x2e1b2138, 0x4d2c6dfc, 0x53380d13, 0x650a7354, 0x766a0abb, 0x81c2c92e, 0x92722c85,
    0xa2bfe8a1, 0xa81a664b, 0xc24b8b70, 0xc76c51a3, 0xd192e819, 0xd6990624, 0xf40e3585, 0x106aa070,
    0x19a4c116, 0x1e376c08, 0x2748774c, 0x34b0bcb5, 0x391c0cb3, 0x4ed8aa4a, 0x5b9cca4f, 0x682e6ff3,
    0x748f82ee, 0x78a5636f, 0x84c87814, 0x8cc70208, 0x90befffa, 0xa4506ceb, 0xbef9a3f7, 0xc67178f2,
];

const IV: [u32; 8] = [
    0x6a09e667, 0xbb67ae85, 0x3c6ef372, 0xa54ff53a, 0x510e527f, 0x9b05688c, 0x1f83d9ab, 0x5be0cd19,
];

/// One compression round over a 64-byte block.
const fn compress(state: &mut [u32; 8], block: &[u8; 64]) {
    let mut w = [0u32; 64];
    let mut i = 0;
    while i < 16 {
        w[i] = u32::from_be_bytes([
            block[4 * i],
            block[4 * i + 1],
            block[4 * i + 2],
            block[4 * i + 3],
        ]);
        i += 1;
    }
    while i < 64 {
        let x = w[i - 15];
        let y = w[i - 2];
        let s0 = x.rotate_right(7) ^ x.rotate_right(18) ^ (x >> 3);
        let s1 = y.rotate_right(17) ^ y.rotate_right(19) ^ (y >> 10);
        w[i] = w[i - 16]
            .wrapping_add(s0)
            .wrapping_add(w[i - 7])
            .wrapping_add(s1);
        i += 1;
    }

    let mut a = state[0];
    let mut b = state[1];
    let mut c = state[2];
    let mut d = state[3];
    let mut e = state[4];
    let mut f = state[5];
    let mut g = state[6];
    let mut h = state[7];

    let mut i = 0;
    while i < 64 {
        let s1 = e.rotate_right(6) ^ e.rotate_right(11) ^ e.rotate_right(25);
        let ch = (e & f) ^ ((!e) & g);
        let t1 = h
            .wrapping_add(s1)
            .wrapping_add(ch)
            .wrapping_add(K[i])
            .wrapping_add(w[i]);
        let s0 = a.rotate_right(2) ^ a.rotate_right(13) ^ a.rotate_right(22);
        let maj = (a & b) ^ (a & c) ^ (b & c);
        let t2 = s0.wrapping_add(maj);

        h = g;
        g = f;
        f = e;
        e = d.wrapping_add(t1);
        d = c;
        c = b;
        b = a;
        a = t1.wrapping_add(t2);
        i += 1;
    }

    state[0] = state[0].wrapping_add(a);
    state[1] = state[1].wrapping_add(b);
    state[2] = state[2].wrapping_add(c);
    state[3] = state[3].wrapping_add(d);
    state[4] = state[4].wrapping_add(e);
    state[5] = state[5].wrapping_add(f);
    state[6] = state[6].wrapping_add(g);
    state[7] = state[7].wrapping_add(h);
}

/// The SHA-256 digest of `data`.
///
/// `const fn` so that the fixture digest is a compile-time constant: a fixture
/// edited without editing the expectation then fails the build, not a test.
pub(crate) const fn digest(data: &[u8]) -> [u8; 32] {
    let mut h = IV;

    let n = data.len();
    let mut i = 0;
    while i + 64 <= n {
        let mut block = [0u8; 64];
        let mut j = 0;
        while j < 64 {
            block[j] = data[i + j];
            j += 1;
        }
        compress(&mut h, &block);
        i += 64;
    }

    // Padding: 0x80, then zeros, then the 64-bit big-endian bit length. One block
    // when the remainder plus the marker plus eight length bytes fits in 64.
    let rem = n - i;
    let mut tail = [0u8; 128];
    let mut j = 0;
    while j < rem {
        tail[j] = data[i + j];
        j += 1;
    }
    tail[rem] = 0x80;
    let blocks: usize = if rem + 1 + 8 <= 64 { 1 } else { 2 };
    let last = blocks * 64;
    let bit_len: u64 = (n as u64) << 3;
    let len_bytes = bit_len.to_be_bytes();
    let mut k = 0;
    while k < 8 {
        tail[last - 8 + k] = len_bytes[k];
        k += 1;
    }

    let mut b = 0;
    while b < blocks {
        let mut block = [0u8; 64];
        let mut q = 0;
        while q < 64 {
            block[q] = tail[b * 64 + q];
            q += 1;
        }
        compress(&mut h, &block);
        b += 1;
    }

    let mut out = [0u8; 32];
    let mut lane = 0;
    while lane < 8 {
        let bytes = h[lane].to_be_bytes();
        let mut t = 0;
        while t < 4 {
            out[4 * lane + t] = bytes[t];
            t += 1;
        }
        lane += 1;
    }
    out
}

/// Lowercase hexadecimal of a 32-byte digest, sixty-four bytes, no allocation.
pub(crate) fn hex(digest_bytes: [u8; 32]) -> [u8; 64] {
    const DIGITS: [u8; 16] = *b"0123456789abcdef";
    let mut out = [0u8; 64];
    let mut i = 0;
    while i < 32 {
        let byte = digest_bytes[i];
        out[2 * i] = DIGITS[usize::from(byte >> 4)];
        out[2 * i + 1] = DIGITS[usize::from(byte & 0x0f)];
        i += 1;
    }
    out
}

#[cfg(test)]
mod tests {
    use super::{digest, hex};

    fn hex_of(input: &[u8]) -> [u8; 64] {
        hex(digest(input))
    }

    fn assert_kat(input: &[u8], expected: &str) {
        let got = hex_of(input);
        assert_eq!(core::str::from_utf8(&got).unwrap(), expected);
    }

    #[test]
    fn known_answer_empty() {
        assert_kat(b"", "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855");
    }

    #[test]
    fn known_answer_abc() {
        assert_kat(
            b"abc",
            "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad",
        );
    }

    #[test]
    fn known_answer_two_block_message() {
        assert_kat(
            b"abcdbcdecdefdefgefghfghighijhijkijkljklmklmnlmnomnopnopq",
            "248d6a61d20638b8e5c026930c3e6039a33ce45964ff2167f6ecedd419db06c1",
        );
    }

    #[test]
    fn known_answer_quick_brown_fox() {
        assert_kat(
            b"The quick brown fox jumps over the lazy dog",
            "d7a8fbb307d7809469ca9abcb0082e4f8d5651e46d3cdb762d02d0bf37c9e592",
        );
    }

    #[test]
    fn block_boundaries_pad_correctly() {
        // 55 bytes is the last remainder that fits one padded block; 56 needs two.
        assert_eq!(digest(&[b'a'; 55]).len(), 32);
        assert_eq!(digest(&[b'a'; 56]).len(), 32);
        assert_eq!(digest(&[b'a'; 64]), digest(&[b'a'; 64]));
        assert_ne!(digest(&[b'a'; 63]), digest(&[b'a'; 64]));
    }

    #[test]
    fn hex_is_lowercase_and_bijective_on_one_byte_patterns() {
        let mut d = [0u8; 32];
        let mut i = 0;
        while i < 32 {
            d[i] = u8::try_from(i).unwrap();
            i += 1;
        }
        let text = hex(d);
        assert_eq!(
            core::str::from_utf8(&text).unwrap(),
            "000102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f"
        );
    }
}
