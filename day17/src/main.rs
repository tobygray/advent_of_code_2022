

enum Piece {
    // ####
    Horizontal,
    // .#.
    // ###
    // .#.    
    Cross,
    // ..#
    // ..#
    // ###    
    L,
    // #
    // #
    // #
    // #    
    Vertical,
    // ##
    // ##
    Square,
}

fn main() -> eyre::Result<()> {
    let jet_pattern = read_jet_pattern()?;
    // TODO: Read input sequence
    // Simulate rock falls
    let rock_count: u64 = 2022;
    for i in 0..rock_count {
        println!("{i}");
    }
    Ok(())
}
