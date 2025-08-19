use core::fmt::Write as _;
// https://en.wikipedia.org/wiki/MD5
const S: [u32; 64] = [
    7, 12, 17, 22, 7, 12, 17, 22, 7, 12, 17, 22, 7, 12, 17, 22, 5, 9, 14, 20, 5, 9, 14, 20, 5, 9,
    14, 20, 5, 9, 14, 20, 4, 11, 16, 23, 4, 11, 16, 23, 4, 11, 16, 23, 4, 11, 16, 23, 6, 10, 15,
    21, 6, 10, 15, 21, 6, 10, 15, 21, 6, 10, 15, 21,
];
#[expect(clippy::unreadable_literal, reason = "doesnt need to be readable")]
const K: [u32; 64] = [
    0xd76aa478, 0xe8c7b756, 0x242070db, 0xc1bdceee, 0xf57c0faf, 0x4787c62a, 0xa8304613, 0xfd469501,
    0x698098d8, 0x8b44f7af, 0xffff5bb1, 0x895cd7be, 0x6b901122, 0xfd987193, 0xa679438e, 0x49b40821,
    0xf61e2562, 0xc040b340, 0x265e5a51, 0xe9b6c7aa, 0xd62f105d, 0x02441453, 0xd8a1e681, 0xe7d3fbc8,
    0x21e1cde6, 0xc33707d6, 0xf4d50d87, 0x455a14ed, 0xa9e3e905, 0xfcefa3f8, 0x676f02d9, 0x8d2a4c8a,
    0xfffa3942, 0x8771f681, 0x6d9d6122, 0xfde5380c, 0xa4beea44, 0x4bdecfa9, 0xf6bb4b60, 0xbebfbc70,
    0x289b7ec6, 0xeaa127fa, 0xd4ef3085, 0x04881d05, 0xd9d4d039, 0xe6db99e5, 0x1fa27cf8, 0xc4ac5665,
    0xf4292244, 0x432aff97, 0xab9423a7, 0xfc93a039, 0x655b59c3, 0x8f0ccc92, 0xffeff47d, 0x85845dd1,
    0x6fa87e4f, 0xfe2ce6e0, 0xa3014314, 0x4e0811a1, 0xf7537e82, 0xbd3af235, 0x2ad7d2bb, 0xeb86d391,
];

// Hilfsfunktionen
const fn left_rotate(x: u32, c: u32) -> u32 {
    (x << c) | (x >> (32 - c))
}

fn md5_padding(message: &mut Vec<u8>) {
    let message_len_bits = (message.len() as u64) * 8;
    message.push(0x80);

    while (message.len() % 64) != 56 {
        message.push(0);
    }

    message.extend_from_slice(&message_len_bits.to_le_bytes());
}
#[expect(clippy::many_single_char_names, reason = "this is the algorithm")]
fn md5_transform(chunk: &[u8], hash: &mut [u32; 4]) {
    let mut a = hash[0];
    let mut b = hash[1];
    let mut c = hash[2];
    let mut d = hash[3];

    let mut m = [0_u32; 16];
    for (i, val) in m.iter_mut().enumerate() {
        *val = u32::from_le_bytes([
            chunk[i * 4],
            chunk[i * 4 + 1],
            chunk[i * 4 + 2],
            chunk[i * 4 + 3],
        ]);
    }

    for i in 0..64 {
        let (f, g) = match i {
            0..=15 => ((b & c) | (!b & d), i),
            16..=31 => ((d & b) | (!d & c), (5 * i + 1) % 16),
            32..=47 => (b ^ c ^ d, (3 * i + 5) % 16),
            48..=63 => (c ^ (b | !d), (7 * i) % 16),
            _ => unreachable!(),
        };

        let temp = d;
        d = c;
        c = b;
        b = b.wrapping_add(left_rotate(
            a.wrapping_add(f).wrapping_add(K[i]).wrapping_add(m[g]),
            S[i],
        ));
        a = temp;
    }

    hash[0] = hash[0].wrapping_add(a);
    hash[1] = hash[1].wrapping_add(b);
    hash[2] = hash[2].wrapping_add(c);
    hash[3] = hash[3].wrapping_add(d);
}

pub fn md5(message: &str) -> String {
    #[expect(clippy::unreadable_literal, reason = "doesnt need to be readable")]
    let mut hash = [
        0x67452301_u32,
        0xefcdab89_u32,
        0x98badcfe_u32,
        0x10325476_u32,
    ];
    let mut message_bytes = message.as_bytes().to_vec();

    // Padding hinzufügen
    md5_padding(&mut message_bytes);

    // Nachricht in 512-Bit-Blöcke (64-Byte-Blöcke) aufteilen
    for chunk in message_bytes.chunks(64) {
        md5_transform(chunk, &mut hash);
    }

    // Endgültiges Ergebnis als Hexadezimalzeichenfolge formatieren
    hash.iter()
        .flat_map(|h| h.to_le_bytes())
        .fold(String::new(), |mut output, b| {
            let _ = write!(output, "{b:02x}");
            output
        })
}
