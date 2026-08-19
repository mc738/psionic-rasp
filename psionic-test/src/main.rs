use psionic_runtime::{Runtime, RuntimeConfiguration, RuntimeConfigurationBuilder};




fn main() {

    let cfg =
        RuntimeConfigurationBuilder::new()
            .with_on_update(Box::new(| ctx | {

            })).build();


    let runtime = Runtime::create(cfg);

    runtime.run();

    println!("Hello, world!");
}
