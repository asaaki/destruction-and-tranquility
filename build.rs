fn main() -> shadow_rs::SdResult<()> {
    if cfg!(windows) {
        static_vcruntime::metabuild();
    }

    shadow_rs::new()
}
