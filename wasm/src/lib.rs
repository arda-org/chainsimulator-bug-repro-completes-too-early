// Code generated by the multiversx-sc build system. DO NOT EDIT.

////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

// Init:                                 1
// Upgrade:                              1
// Endpoints:                            1
// Async Callback:                       1
// Total number of exported functions:   4

#![no_std]

multiversx_sc_wasm_adapter::allocator!();
multiversx_sc_wasm_adapter::panic_handler!();

multiversx_sc_wasm_adapter::endpoints! {
    contract
    (
        init => init
        upgrade => upgrade
        issue => issue
    )
}

multiversx_sc_wasm_adapter::async_callback! { contract }
