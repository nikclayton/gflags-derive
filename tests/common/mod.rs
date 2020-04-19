use gflags;
use std::any::{Any, TypeId};
use std::collections::HashMap;

/// `ExpectedFlag` describes the expected state of a flag. Individual tests
/// create one of these and pass to `check_flag` to confirm that the actual
/// state matches the expected state.
pub struct ExpectedFlag<'a, T: std::marker::Sized + Any> {
    pub doc: &'static [&'static str],
    pub name: &'static str,
    pub placeholder: Option<&'static str>,
    pub generated_flag: &'a gflags::Flag<T>,
}

/// Fetch flag definitions from the registry and return them as a `HashSet`
/// so individual flags can be checked.
pub fn fetch_flags() -> HashMap<&'static str, &'static gflags::registry::Flag> {
    let mut flags: HashMap<&'static str, &gflags::registry::Flag> = HashMap::new();

    // The registry doesn't provide methods to fetch individual entries, just
    // an iterator, so create a map of flags keyed by flag name
    for flag in gflags::inventory::iter::<gflags::registry::Flag> {
        flags.insert(flag.name, flag);
    }

    flags
}

/// Performs various assertions to confirm that the flag in `got` matches
/// the expectations in `want`.
pub fn check_flag<T: 'static>(
    want: Option<ExpectedFlag<'static, T>>,
    got: Option<&gflags::registry::Flag>,
) {
    if want.is_none() && got.is_none() {
        return;
    }

    assert_eq!(
        want.is_none() && got.is_some(),
        false,
        "Unexpected flag with name --{}",
        got.unwrap().name
    );

    assert_eq!(
        want.is_some() && got.is_none(),
        false,
        "Failed to find flag with name --{}",
        want.unwrap().name
    );

    let want = want.unwrap();
    let got: &gflags::registry::Flag = got.unwrap();

    assert_eq!(want.doc, got.doc);

    assert_eq!(want.placeholder, got.placeholder);

    // Technically this type checking isn't necessary, because if the type
    // parameter used to construct `ExpectedFlag` doesn't match the type of
    // the generated flag it's a compile time error and the test won't compile
    // I'm keeping the code here as an example of how to do this.
    let typed_flag: gflags::Flag<T> = gflags::Flag::null();
    assert!(is_same_type(&typed_flag, want.generated_flag));
}

/// True if both arguments are the same type
fn is_same_type<S: ?Sized + std::any::Any, T: ?Sized + std::any::Any>(_s: &S, _t: &T) -> bool {
    TypeId::of::<S>() == TypeId::of::<T>()
}
