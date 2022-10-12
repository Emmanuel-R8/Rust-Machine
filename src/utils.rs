use crate::world::world::World;

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
    fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "{}[{}][{}] {}",
                chrono::Local::now().format("[%Y-%m-%d][%H:%M:%S]"),
                record.target(),
                record.level(),
                message
            ))
        })
        .level(log::LevelFilter::Debug)
        // In case we want to echo to stdout
        // .chain(std::io::stdout())
        .chain(fern::log_file(filename)?)
        .apply()?;
    return Ok(());
}

// Close world
pub fn close_world(world: World) {}
