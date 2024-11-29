use ruid_set::{prefix::Prefix, ruid::RuidGenerator};


fn main() {

    // RuidGenerator を初期化
    let mut generator = RuidGenerator::new()
        .set_default_device_id(0x1234)
        .set_prefix(Prefix::UncategorizedData);

    // RUID を生成
    let ruid = generator.generate();

    println!("Generated RUID: {}", ruid.to_string());
}
