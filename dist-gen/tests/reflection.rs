use std::io::Read;

#[path = "../../schema/reflection_generated.rs"]
mod fbs_reflection;
use fbs_reflection::reflection;

#[test]
fn reflection() -> std::io::Result<()> {
    // Test loading a monster rep using reflection.fbs
    println!("{:?}", std::env::current_dir()?);

    let mut f = std::fs::File::open("../schema/monster.bfbs").unwrap();
    let mut buf = Vec::new();
    f.read_to_end(&mut buf).expect("file reading failed");

    let monster_schema = reflection::get_root_as_schema(&buf[..]);

    for obj in monster_schema.objects().iter()  {
        println!("Object = {}", obj.name());
    }

    Ok(())
}