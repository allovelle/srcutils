mod dbg;
mod offset;

pub use {dbg::*, offset::*};

/// A logical line and charcter position in a source file. Does not correlate to
/// byte offset & length. For byte offset & length, use [crate::SrcOffset].
#[derive(Debug, Clone, Copy, Default)]
pub struct SrcPt(u32);

#[derive(Debug, Clone, Copy, Default)]
pub struct Sentinel
{
    pub total_chars: u32, // grows with every character
    pub total_lines: u32, // grows with every newline
}

impl Sentinel
{
    pub fn new() -> Self
    {
        Self { total_chars: 0, total_lines: 0 }
    }

    /// Number of bits required to represent the largest possible column.
    #[inline]
    fn col_bits(&self) -> u8
    {
        match self.total_chars
        {
            0 => 1, // need at least 1 bit
            n => 32 - n.leading_zeros() as u8,
        }
    }

    #[inline]
    fn col_mask(&self) -> u32
    {
        (1u32 << self.col_bits()) - 1
    }

    /// Encode a (line, col) pair into a single 32-bit SrcPt.
    /// Assumes line <= total_lines and col <= total_chars.
    pub fn encode(&self, line: u32, col: u32) -> SrcPt
    {
        let cb = self.col_bits();
        SrcPt((line << cb) | (col & self.col_mask()))
    }

    /// Decode a SrcPt back into (line, col).
    /// Always uses the *current* sentinel state, which is safe.
    pub fn decode(&self, pt: SrcPt) -> (u32, u32)
    {
        let cb = self.col_bits();
        let line = pt.0 >> cb;
        let col = pt.0 & self.col_mask();
        (line, col)
    }

    /// Add a character can be a logical new line.
    pub fn push_char(&mut self)
    {
        self.total_chars += 1;
    }

    /// Add a newline (also a character).
    pub fn push_newline(&mut self)
    {
        self.total_chars += 1;
        self.total_lines += 1;
    }
}

// use num_traits::int::PrimInt;
// use std::ops::AddAssign;

// pub trait Integral: PrimInt + Default + AddAssign {}

// /// Either one of these fields will not exceed N::max_value(). Therefore, they
// /// can be used to represent a _logical_ point in a source file. Lines and chars
// /// increase proportionally to each other.
// #[derive(Debug, Clone, Copy, Default)]
// struct Sentinel<N: Integral>
// {
//     /// Count of `\n` characters
//     lines: N,
//     /// Utf8 codepoints, not bytes, therefore `chars * 4 = bytes`
//     chars: N,
// }

// impl<N: Integral> Sentinel<N>
// {
//     #[rustfmt::skip]
//     pub fn new() -> Self { Self::default() }

//     #[rustfmt::skip]
//     #[inline(always)]
//     pub fn add_lines(&mut self, lines: N) { self.lines += lines; }

//     #[rustfmt::skip]
//     #[inline(always)]
//     pub fn inc_line(&mut self) { self.lines += N::one(); }

//     #[rustfmt::skip]
//     #[inline(always)]
//     pub fn add_chars(&mut self, chars: N) { self.chars += chars; }

//     #[rustfmt::skip]
//     #[inline(always)]
//     pub fn inc_char(&mut self) { self.chars += N::one(); }

//     #[rustfmt::skip]
//     #[inline(always)]
//     pub fn line_at(&self, point: SrcPoint<N>) -> N {
//         let mask = bits_required(self.chars);
//         self.lines
//     }

//     #[rustfmt::skip]
//     #[inline(always)]
//     pub fn char_at(&self, point: SrcPoint<N>) -> N { self.chars }
// }

// /// Number of bits required to represent `n`.
// /// Returns 0 for n == 0, otherwise returns 1..=usize::BITS.
// fn bits_required(n: usize) -> usize
// {
//     // ilog2 panics on 0 so return 0 if zero
//     (if n == 0 { 0 } else { n.ilog2() + 1 }) as usize
// }

// #[derive(Debug, Clone, Copy, Default)]
// struct SrcPoint<N: Integral>
// {
//     buffer: N,
// }

// impl<N: Integral> SrcPoint<N>
// {
//     pub(self) fn from_buf(buffer: N) -> Self
//     {
//         SrcPoint { buffer }
//     }

//     pub fn new(lines: N, chars: N) -> Self
//     {
//         let buffer = (lines << 8) | chars;
//         SrcPoint { buffer }
//     }
// }

// #[cfg(test)]
// mod tests
// {
//     use super::*;

//     #[test]
//     fn test_src_point()
//     {
//         // let point = SrcPoint::new(0u8);
//         // assert_eq!(point.buffer, 42);
//     }
// }
