use crate::bindings::{
    exports::wasvy::ecs::guest::{Guest, GuestSystem, System},
    wasvy::ecs::app::App,
};

#[allow(warnings)]
mod bindings;

struct ModSystem(Box<dyn Fn(String) -> String>);

impl GuestSystem for ModSystem {
    fn run(&self, input: String) -> String {
        (self.0)(input)
    }
}

struct GuestComponent;

impl Guest for GuestComponent {
    type System = ModSystem;

    fn setup() {
        println!("Setup start");

        let app = App::new();

        // A boxed system
        let mod_system = ModSystem(Box::new(|input: String| {
            format!("Ran system with input {input}")
        }));

        // An exported system, but still not what add_system expects
        let export_system = System::new(mod_system);

        // There has to be a better way of doing this... but casting should be fine since both implementations match
        let system = unsafe {
            bindings::wasvy::ecs::guest::System::from_handle(export_system.take_handle())
        };

        // Test that this system can be run
        system.run("test");

        app.add_system(system);

        println!("Setup end");
    }
}

bindings::export!(GuestComponent with_types_in bindings);
