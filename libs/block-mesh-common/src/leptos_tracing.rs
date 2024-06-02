use std::sync::Once;

// https://rustwasm.github.io/wasm-bindgen/reference/passing-rust-closures-to-js.html
#[inline]
pub fn setup_leptos_tracing() {
    static SET_HOOK: Once = Once::new();
    SET_HOOK.call_once(|| {
        tracing_wasm::set_as_global_default();
        /*
        tracing_subscriber::registry()
            .with(EnvFilter::try_from_default_env().unwrap_or_else(|_| "info".into()))
            .with(
                tracing_subscriber::fmt::layer()
                    .with_ansi(false)
                    .without_time(),
            )
            .init();
         */
    });
}