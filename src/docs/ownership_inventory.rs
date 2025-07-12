#[cfg(test)]
mod ownership_inventory {
    mod module_3 {
        #[test]
        fn struct_fields() {
            struct TestResult {
                /// Student's scores on a test
                scores: Vec<usize>,

                /// A possible value to curve all scores
                curve: Option<usize>,
            }

            impl TestResult {
                pub fn get_curve(&self) -> &Option<usize> {
                    &self.curve
                }

                /// If there is a curve, then increments all scores by the curve
                pub fn apply_curve(&mut self) {
                    // if let Some(curve) = self.get_curve() {
                    // ^ ERROR: self.get_curve already borrows the whole Self immutably, and hence iter_mut
                    // cannot borrow self mutably. Instead, if we use self.curve, compiler will know
                    // that we are doing partial borrow limited to curve.
                    // NOTE: This is a commonly used approach for borrow checker issues with structs.
                    if let Some(curve) = self.curve {
                        for score in self.scores.iter_mut() {
                            // *score += *curve;
                            *score += curve;
                        }
                    }
                }
            }
        }

        #[test]
        fn sliced_refs() {
            fn find_nth<T: Ord + Clone>(elems: &[T], n: usize) -> T {
                // elems.sort();
                /* ^ ERROR: cannot borrowed as mutable since elems is behind & ref
                 * impl<T> [T] pub fn sort(&mut self) where T: Ord, */

                let mut elem_refs: Vec<&T> = elems.iter().collect();
                // collect the refs in another vector and sort that instead
                elem_refs.sort();

                let t = &elems[n];
                return t.clone();
            }
        }
    }

    mod module_4 {
        #[test]
        fn lifetime_annotation_basic() {
            fn concat_all(
                iter: impl Iterator<Item = String>,
                s: &str,
            ) -> impl Iterator<Item = String> {
                // iter.map(move |s2| s2 + s)

                /* ^Error: breaks because lifetime captured with s isn't appearing the bounds of the
                 * returned type. Compiler needs to know that since you are using s: &str, the
                 * returned value lives long enough as s lives.*/

                let s = s.to_owned(); // So either we can own this and use the &str from
                                      // the stack memory OR annotate the lifetimes (see concat_all2)
                iter.map(move |s2| s2 + &s)
            }

            // NOTE: Above example with lifetime annotation, which is better because this doesn't
            // have any runtime overhead like to_owned has
            fn concat_all2<'a>(
                iter: impl Iterator<Item = String> + 'a,
                s: &'a str,
            ) -> impl Iterator<Item = String> + 'a {
                iter.map(move |s2| s2 + s)
            }
        }

        #[test]
        fn lifetime_annotation_trait_objects() {
            fn trait_object() {
                use std::fmt::Display;

                // fn add_displayable<T: Display>(v: &mut Vec<Box<dyn Display>>, t: T) {

                /* ^ Error: This breaks because when a trait object from `t` is pushed in the vector,
                 * compiler wants to ensure that the trait object outlives the vector. */
                fn add_displayable<'a, T: Display + 'a>(v: &mut Vec<Box<dyn Display + 'a>>, t: T) {
                    // NOTE: that we have specified lifetime for the trait object and not the vec mut ref
                    v.push(Box::new(t));
                }
            }
        }
    }
}
