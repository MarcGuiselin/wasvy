use wasmtime::{
    Result,
    component::{Resource, bindgen},
};
use wasmtime_wasi::{ResourceTable, WasiCtx, WasiCtxView, WasiView};

bindgen!({
    path: "./wit/ecs/ecs.wit",
    world: "host",
    // Interactions with `ResourceTable` can possibly trap so enable the ability
    // to return traps from generated functions.
    imports: { default: trappable },
    with: {
        "wasvy:ecs/app/app": App,
    },
});
use crate::wasvy::ecs::app as bindings;

struct HostStates {
    // These two are required basically as a standard way to enable the impl of IoView and
    // WasiView.
    pub wasi_ctx: WasiCtx,
    pub resource_table: ResourceTable,
    // You can add other custom host states if needed
}

impl WasiView for HostStates {
    fn ctx(&mut self) -> WasiCtxView<'_> {
        WasiCtxView {
            ctx: &mut self.wasi_ctx,
            table: &mut self.resource_table,
        }
    }
}

impl bindings::Host for HostStates {}
impl bindings::HostApp for HostStates {
    fn new(&mut self) -> Result<Resource<App>> {
        let res = self.resource_table.push(App)?;
        Ok(res)
    }

    fn drop(&mut self, _rep: Resource<App>) -> Result<()> {
        Ok(())
    }

    fn add_system(
        &mut self,
        _app: Resource<App>,
        system: Resource<bindings::System>,
    ) -> Result<()> {
        // TODO: No clue how to get the system from here to run. We can't do `system.run("string")`
        // Since system is defined by the guest import, it's not stored in a ResourceTable that we can fetch it from

        Ok(())
    }
}

pub struct App;
