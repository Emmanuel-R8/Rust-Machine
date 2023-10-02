
// Instructions are normally executed in the order in which they are stored in
// memory. Since full-word instructions cannot cross word boundaries, it would
// occasionally be necessary to insert a no-op instruction in places where a
// full-word instruction or constant followed a half-word instruction that did not
// fall on an odd halfword address. This costs address space, I Cache space, and
// possibly execution time to execute the no-op.

// ******
// NOTE: A full word address is split between an __even__ half-world address,
// followed by an __odd__ half-word address.
// ******

// The cdr code field of each word executed contains sequencing information to
// minimize this waste. The cdr code takes on one of four values, which specify how
// much the PC is incremented after executing an instruction from this word. Note
// that the PC contains a half-word address.

// | CDR Code | PC Increment  | Comment                 |
// |:----------:|:---------:|:----------------------------|
// | 0  | +1  | Normal instruction sequencing                   |
// | 1  | Illegal  | Fence; marks end of compiled function                |
// | 2   | -1 | On some constants. |
// | 3   | +2 PC even | Before some constants, on some constants |
// |   | +3 PC odd |  |
// :CDR header {tbl-colwidths="[15, 15, 70]"}

pub struct Instructions {

}
