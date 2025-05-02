pub struct ModManager {
    rhai_engine: RhaiEngine,
    wasm_runtime: WasmRuntime,
    native_plugins: Vec<Box<dyn Plugin>>,
    registered_hooks: HashMap<HookType, Vec<HookCallback>>,
}
