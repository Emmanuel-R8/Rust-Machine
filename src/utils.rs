use super::world::world::World;

#[inline]
pub fn ldb(ss: u8, pp: u8, source: u32) -> u32 {
    (source >> pp) & ((1 << ss) - 1)
}

#[inline]
pub fn dpb(field: u32, ss: u8, pp: u8, background: u32) -> u32 {
    ((field & ((1 << ss) - 1)) << pp) | (background & !(((1 << ss) - 1) << pp))
}

#[inline]
pub fn ceiling(n: u32, d: u32) -> u32 {
    (n + (d - 1)) / d
}

///////////////////////////////////////////////////////////////////////////////////////////////////
//
// Logging
//
///////////////////////////////////////////////////////////////////////////////////////////////////

fn setup_logger(filename: &str) -> Result<(), fern::InitError> {
    fern::Dispatch
        ::new()
        .format(|out, message, record| {
            out.finish(
                format_args!(
                    "{}[{}][{}] {}",
                    chrono::Local::now().format("[%Y-%m-%d][%H:%M:%S]"),
                    record.target(),
                    record.level(),
                    message
                )
            )
        })
        .level(log::LevelFilter::Debug)
        // In case we want to echo to stdout
        // .chain(std::io::stdout())
        .chain(fern::log_file(filename)?)
        .apply()?;
    return Ok(());
}

// Close world
pub fn close_world(_world: World) {}

#[inline]
pub fn byte_swap_32(val_u32: u32) -> u32 {
    let b1 = val_u32 & 0x_______ff;
    let b2 = val_u32 & 0x_____ff00;
    let b3 = val_u32 & 0x__ff_0000;
    let b4 = val_u32 & 0xff00_0000;
    return (b1 << 24) | (b2 << 8) | (b3 >> 8) | (b4 >> 24);
}

#[inline]
pub fn unpack_32_to_8(val_u32: u32) -> [u8; 4] {
    let b1 = (val_u32 & 0xff00_0000) >> 24;
    let b2 = (val_u32 & 0x__ff_0000) >> 16;
    let b3 = (val_u32 & 0x_____ff00) >> 8;
    let b4 = (val_u32 & 0x_______ff) >> 0;

    return [b1 as u8, b2 as u8, b3 as u8, b4 as u8];
}

#[inline]
pub fn pack_8_to_32(bytes: [u8; 4]) -> u32 {
    let b1 = (bytes[0] as u32) << 24;
    let b2 = (bytes[1] as u32) << 16;
    let b3 = (bytes[2] as u32) << 8;
    let b4 = (bytes[3] as u32) << 0;

    return b1 + b2 + b3 + b4;
}
