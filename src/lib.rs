pub mod bounds2;

// Remove this when all items are complete.
#[cfg(not(debug_assertions))]
compile_error!(r###"Complete TODO List:
[ ] Implementations for the following types:
    FixedArray
    Line, Grid2, Grid3, (maybe): Grid4
    Segment, OffsetGrid2, OffsetGrid3, (maybe): OffsetGrid4
    Scroll, Scroll2, Scroll3, (maybe): Scroll4
    Bounds, Bounds2, Bounds3, (maybe): Bounds4
[ ] Write documentation.
[ ] Write readme.
[ ] Provide examples.
"###);

mod error_messages {
    macro_rules! error_messages {
        ($($name:ident = $message:literal;)*) => {
            $(
                pub const $name: panicmsg::PanicMsg = panicmsg::PanicMsg::new($message);
            )*
        };
    }
    error_messages!(
        UNALLOCATED_BUFFER      = "Buffer was not allocated.";
        X_MAX_EXCEEDS_MAXIMUM   = "X max bound exceeds maximum.";
        Y_MAX_EXCEEDS_MAXIMUM   = "Y max bound exceeds maximum.";
        Z_MAX_EXCEEDS_MAXIMUM   = "Z max bound exceeds maximum.";
        NOT_ALLOCATED           = "Not allocated.";
        SIZE_TOO_LARGE          = "Size is too large.";
        OUT_OF_BOUNDS           = "Out of bounds.";
        INDEX_OUT_OF_BOUNDS     = "Index is out of bounds.";
        AREA_IS_ZERO            = "Width/Height cannot be 0.";
        VOLUME_IS_ZERO          = "Width/Height/Depth cannot be 0.";
        INFLATE_OVERFLOW        = "Inflate operation results in integer overflow.";
        DEFLATE_OVERFLOW        = "Deflate operation results in integer overflow.";
        RESIZE_OVERFLOW         = "Resize operation results in overflow.";
    );
}