// All the fuss about str in a single doc
#[cfg(test)]
mod str_fuss {
    /*
    `str` is **Unsized primitive type** (represents utf-8 encoded strings)
     Other examples of Unsized are `[T]`, `str`, `dyn Trait`.

     #[lang = "str"] // this tells it is compiler iternals
     struct str {  } // stores u8 but guarantees utf-8

     And like all other unsized types you can't work with them directly,
     instead you use fat pointers to get hold of them {Dynamically Sized Type (DST) mechanics}
     Hence, &str isn't just a pointer to a str struct, instead it's a fat pointer.
     (
        *const u8, // actual memory address of the first u8 byte
        usize // number of bytes (not characters!)
     )

     Import point to note here is that technically str actually leads to u8 slice i.e. [u8]
     just that the u8's in str are bound to be utf-8 compatible and the invariant
     is maintained and assumed everywhere in the compiler as well.

     `.chars()` Iterator on &str emits chars, which won't just iterate on byte level offsets
     but rather valid utf-8 chars! for byte level iteration, we can use `.bytes()`

     indexing on &str is invalid because of utf-8 guarantee violations

     `.as_ptr()` gives `*const u8`, access to the actual memory address of the first byte

     // let s: &str = "hello";
     // let p: *const u8 = s.as_ptr();

     All string literals have `static lifetime since they are heap allocated
     (and stored in a read-only memory) and they stay alive till the program completes
    */

    /* ====================================================================
     * Quick NOTE about == fat pointers ==
     * low-level compiler generated representation of a pointer which
     * carriers some metadata along with it
     *
     * Eg.
     * # fat pointer for &[T] (reference to a slice of T array)
     *   (
     *      data_ptr,  // actual memory address,
     *      length     // length of the slice
     *   )
     *
     * # fat pointer of &dyn Trait (reference to trait object)
     *   (
     *      data_ptr,  // actual memory address of the underlying trait object,
     *      vtable_ptr // used for dynamic dispatch
     *   )
     * ====================================================================
     */

    /*
     `String` on the other-hand is heap allocated vector encapsulation of u8 with utf-8 guarantees
     struct String {
       vec: Vec<u8>;
     }

     We can pass a &String even when &str is expected &str because String struct
     implements Deref trait which allows String to be dereferenced to some other type.
     This is just a pointer and length pass-through and involves no data copy.

     // impl Deref for String {
     //   type Target = str;
     //   fn deref(&self) -> &Self::Target {
     //       &self[..] // slice of the whole string
     //   }
     // }
    */
}
