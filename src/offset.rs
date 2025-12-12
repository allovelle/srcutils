/// An offset & length position in a source file. Correlates to a byte offset &
/// length assuming multi-byte UTF-8 codepoints. For efficient encoding of
/// logical line & character positions, use [crate::SrcPt].
#[derive(Debug, Clone, Copy, Default)]
pub struct SrcOffset(u32);

pub struct SrcLoc;

/// Efficient storage of offset locations for tokens that nest other tokens.
/// Stores the start token of each parent node with the final node being the
/// token itself.
pub struct SrcPath;
