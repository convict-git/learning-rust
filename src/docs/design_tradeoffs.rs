#[cfg(test)]
mod design_tradeoffs {
    #[test]
    fn references() {
        /*
         * Context: You are designing a simple asset manager for a game engine.
         * Functionality: An API client will provide paths of assets to load, and gets back access to the loaded assets.
         * Assumptions: You may assume that loads happen synchronously and instantaneously.
         * Designs: Below are several proposed designs to implement the functionality.

        pub struct AssetManager {
            // ...
        }

        // Option 1: return a reference
        impl AssetManager {
        pub fn load(&mut self, path: PathBuf) -> &Asset;
        }

        // Option 2: return a mutable reference
        impl AssetManager {
        pub fn load(&mut self, path: PathBuf) -> &mut Asset;
        }

        // Option 3: return a reference-counted pointer
        impl AssetManager {
        pub fn load(&mut self, path: PathBuf) -> Rc<Asset>;
        }

        // Option 4: return a copyable handle that can be turned into a reference
        #[derive(Copy, Clone)]
        pub struct AssetHandle(usize);

        impl AssetManager {
        pub fn load(&mut self, path: PathBuf) -> AssetHandle;
        pub fn get(&self, handle: AssetHandle) -> &Asset;
        }
        */

        /*
        == Requirement == Once loaded, an asset should be permanently immutable.
        == Solution == Option 1, 3, 4 - They return immutable borrow of the asset. Rc too is a
        multi-borrowed immutable ownership of the underlying value.
        */

        /*
        == Requirement == Clients of the asset manager need to retain access to assets
        across several short-term borrows of the asset manager.
        == Solution == Here, "across several short-term borrows of the asset manager" means
        asset manager is going to get allocated and deallocated often, and still we should be
        able to use loaded asset. In Option 1 & 2, the lifetime of the &Asset / &mut Asset is
        tied to the lifetime of &mut self of AssetManager, hence they aren't valid.
        Option 3 returns a Rc<Asset> which is a smart pointer, and lifetime is NOT tied.
        Doubt: in Option 4
        */

        /*
        == Requirement == It is important that all assets be deallocated at a single, predictable time.
        == Solution == Option 1, 2, 4. Here the lifetimes of the all the assets are tied with
        the AssetManager, so all gets deallocated when AssetManager is dropped.
        In Option 3, since the Asset is captured via Rc, AssetManager cannot guarantee of all
        the live strong references of the returned smart pointer.
        */
    }

    #[test]
    fn trait_trees() {
        /*
         * Context: You are designing a simple user interface framework that consists of a tree of widgets, such as text and buttons.
         * Functionality: The API provides a Widget trait that defines how a widget works.
         *                The API client implements the Widget trait, and calls it to render a UI.
         * Designs: Below are several proposed designs to implement the functionality.

        // Option 1: children must be Self
        pub trait Widget: Sized {
        fn render(&self) -> Vec<Self>;
        }

        // Option 2: children are a trait parameter
        pub trait Widget<Children> {
        fn render(&self) -> Vec<Children>;
        }

        // Option 3: children are an associated type
        pub trait Widget {
        type Children: Widget;
        fn render(&self) -> Vec<Self::Children>;
        }

        // Option 4: children are a reference trait object
        pub trait Widget {
        fn render(&self) -> Vec<&dyn Widget>;
        }

        // Option 5: children are a boxed trait object
        pub trait Widget {
        fn render(&self) -> Vec<Box<dyn Widget>>;
        }
        */

        /*
        == Requirement == The API client is expected to provide a single WidgetImpl enum representing all
        possible widgets, and implement the Widget trait for WidgetImpl.
        == Solution == Option 1 is correct since we have only one WidgetImpl enum for all widget types,
        Option 2 is unnecessary because generics aren't useful anymore with single enum capturing all,
        Option 3 again similar to above, Option 4 and 5 No need of dynamic dispatches
        */

        /*
         == Requirement == The API client is expected to provide a unique struct for each possible widget,
         and implement the Widget trait for each struct. Each widget can return a vector containing widgets of any possible type.
        == Solution == Option 4 and 5 can capture trait objects which will allow each possible widget type as part of children vec.
        Option 1, 2 and 3, doesn't allow heterogenous children container.
        */

        /*
        == Requirement == Only types that implement the Widget trait should be returned from render.
        == Solution == Option 1, 3, 4, 5 are correct because they all ensure the return type is Vec contained
        type which implements Widget, but in Option 2, we can have any Children without any bounds on it.
        */
    }

    #[test]
    fn dispatch() {
        /*
        Context: You are designing a simple event system that calls callback functions in response to events.
        Functionality: An Events struct provides functions to register callbacks. A callback is either parallelizable
                       (runnable across multiple threads) or sequential (must be run only on the main thread).
        Designs: Below are several proposed designs to implement the functionality.

           pub struct Events {
               // ...
           }

           // Option 1: parallel and sequential are two separate methods
           impl Events {
               pub fn register<E, F: Fn(E)>(&mut self, f: F) { /* .. */ }
               pub fn register_sequential<E, F: Fn(E)>(&mut self, f: F) { /* .. */ }
           }

           // Option 2: parallel and sequential are two members of an enum
           pub enum Callback<F> {
               Parallel(F),
               Sequential(F)
           }
           impl Events {
               pub fn register<E, F: Fn(E)>(&mut self, f: Callback<F>) { /* .. */ }
           }

           // Option 3: parallel and sequential are markers in a trait method
           pub trait Register<Marker, F, E> {
               fn register(&mut self, f: F);
           }
           pub struct Parallel;
           pub struct Sequential;
           impl<F, E> Register<Parallel, F, E> for Events
           where F: Fn(E) {
               fn register(&mut self, f: F) { /* .. */ }
           }
           impl<F, E> Register<Sequential, F, E> for Events
           where F: Fn(Sequential, E) {
               fn register(&mut self, f: F) { /* .. */ }
           }
        */

        /*
        == Requirement == A callback should be considered parallelizable by default, and the API should reflect that default behavior.
        == Solution ==
        In Option 1:
        events.register(|e: OnClick| { /* .. */ })
        events.register_sequential(|e: OnClick| { /* .. */ })

        In Option 2:
        events.register(Callback::Parallel(|e: OnClick| { /*..*/ }))
        events.register(Callback::Sequential(|e: OnClick| { /*..*/ }))

        In Option 3:
        events.register(|e: OnClick| { /* .. */ })
        events.register(_: Sequential, |e: OnClick| { /* .. */ })

        As, it can be seen, both in Option 1 and Option 3, we need to explicitly use `_sequential` suffix or
        (_: Sequential) Marker for sequential callbacks.
        In Option 2, both parallel and sequential are treated equally
        */

        /*
        == Requirement == The API should export as few methods as possible.
        == Solution ==
        Option 2, and Option 3 exports only a single method.
        */

        /*
        == Requirement == The API should lean on the compiler's type inference system as little as possible.
        == Solution == Option 3 heavily relies on the overloaded `register` method
        */
    }

    #[test]
    fn intermediates() {
        /*
         Context: You are designing a serialization library that converts Rust data types into formats like JSON.
         Functionality: A Serialize trait that can be implemented by serializable types, and a to_json function that converts serializable types into JSON.
         Designs: Below are several proposed designs to implement the functionality.

            // Option 1: types serialize to a `Value` enum
            pub enum Value {
                String(String),
                Number(isize)
            }

            pub trait Serialize {
                fn serialize(&self) -> Value;
            }

            fn value_to_json(value: Value) -> String {
                /* .. */
            }

            pub fn to_json(data: impl Serialize) -> String {
                let value = data.serialize();
                value_to_json(value)
            }


            // Option 2: types serialize via calls to a `Serializer` interface
            pub trait Serialize {
                fn serialize<S: Serializer>(&self, serializer: &mut S);
            }

            pub trait Serializer {
                fn serialize_string(&mut self, s: &str);
                fn serialize_number(&mut self, n: isize);
            }

            struct JsonSerializer { buffer: String };
            impl Serializer for JsonSerializer {
                /* .. */
            }

            pub fn to_json(data: impl Serialize) -> String {
                let mut serializer = JsonSerializer { buffer: String::new() };
                data.serialize(&mut serializer);
                serializer.buffer
            }
        */

        /*
        == Requirement == It should be possible to add a new data format without needing to modify code in existing implementations
                          of the Serialize trait.
        == Solution == For both Option 1 and Option 2, we need to implement Serialize for any new data format
        */

        /*
        == Requirement == The process of serialization should consume as little memory as possible.
        == Solution == For Option 1, data is serialized to intermediate value, and then to a json string
                       whereas, in Option 2, data is serialized directly in the buffered string
         */

        /*
        == Requirement == When the same data type is serialized into multiple formats, that should increase the size
                          of the compiled binary as little as possible.
        == Solution == It is important to understand that Option 2 depends on compile time polymorphism using generics,
                       which will create multiple copies of the code, replacing each generic with a type at compilation
                       (which is called Monomorphization -- polymorphic to monomorphic function conversion).
                       `to_json` has two generics, `Serialize` and `Serializer` whereas in Option 1, there's only one generic, `Serialize`.
        */
    }
}
