mod common;
mod gdtf;
mod mvr;

fn main() -> anyhow::Result<()> {
    mvr::generate_mvr_schema()?;
    gdtf::generate_gdtf_schema()?;

    Ok(())
}
