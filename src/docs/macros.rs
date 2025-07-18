#[cfg(test)]
mod macros {
    // === Macros ==
    // - Declarative
    //   Match-like macro pattern syntax
    //
    // - Procedural
    //   simple theoritically, flow is:
    //   use the TokenStream -> AST -> TokenStream, and all the functionalities of `syn` and `quote`.
    //
    // --- custom (derive)
    // --- attribute-like
    // --- function-like

    // == Derivable Traits from std library ==  Procedural Macros
    // - NOTE: Display trait cannot be derived, must be implemented for "{:?}" formating
    //
    // == Debug trait == use-cases in assert_eq, general debugging, converting your DS to a string
    //   for doing logging, comparisions etc.
    //
    // == PartialEq trait ==  used for == and != with itself. -> should be preferred for assert_eq if implemented
    //
    // == Eq trait ==  no methods, purpose is for saying all instances of a structs/enums implementing
    //   this trait should be treated as equals
    //
    // == PartialOrd trait ==  partial_cmp returns Option<Ordering> <, >, <=, >=.
    //   (if a valid ordering may or may NOT be present, eg. floats and NaN)
    //
    // == Ord ==  cmp return Ordering. If Valid ordering between two values of the type always exist.
    //
    // == Clone trait ==  deep copy of the value (not bit-wise copy)
    //   all fields inside must derive Clone trait as well
    //
    // == Copy trait == bit wise copy, if Copy implemented, it's used as a default Clone too
    //
    // IMPORTANT NOTE: Rust doesn't allow us to implement both Copy and Drop trait for a type at
    // the same time? Why? Because since Copy can lead to bitwise copy of pointers which were
    // meant to be freed(once) using the Drop trait. Copying can lead to freeing the same memory
    // twice.
    //
    // == Hash trait == hash method, all fields must derive Hash trait too
    //    Usages like in HashMap<K, V>, K must implement Hash trait
    //
    // == Default trait == ::default() method, all fields must implement Default trait
    //   usages as simple as ::default() or in Option::<T>::unwrap_or_default(..) where T : Default
}
