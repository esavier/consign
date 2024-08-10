use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let build = vergen::BuildBuilder::all_build()?;
    let cargo = vergen::CargoBuilder::all_cargo()?;
    let rustc = vergen::RustcBuilder::all_rustc()?;
    let si = vergen::SysinfoBuilder::all_sysinfo()?;
    let gix = vergen_gix::GixBuilder::all_git()?;

    vergen::Emitter::default()
        .add_instructions(&build)?
        .add_instructions(&cargo)?
        .add_instructions(&rustc)?
        .add_instructions(&si)?
        .add_instructions(&gix)?
        .emit()?;
    Ok(())
}
