#[allow(static_mut_refs)]
mod bindings;

use crate::bindings::wasi::clocks::wall_clock;
use crate::bindings::exports::test::foo::test_foo::*;
use crate::bindings::test::bar_api::test_bar;
// Import for using common lib (also see Cargo.toml for adding the dependency):
// use common_lib::example_common_function;
use std::cell::RefCell;

/// This is one of any number of data types that our application
/// uses. Golem will take care to persist all application state,
/// whether that state is local to a function being executed or
/// global across the entire program.
struct State {
    total: u64,
}

thread_local! {
    /// This holds the state of our application.
    static STATE: RefCell<State> = RefCell::new(State {
        total: 0,
    });
}

struct TestFooResourceImpl;

impl GuestTestFooResource for TestFooResourceImpl {
    fn new() -> Self {
        TestFooResourceImpl
    }
}

struct Component;

impl Guest for Component {

    type TestFooResource = TestFooResourceImpl;

    fn own_record() -> TestFooRecord {
        TestFooRecord {
            foo: 0
        }
    }

    fn forwarded_record() -> test_bar::TestBarRecord {
        test_bar::TestBarRecord {
            bar: 0
        }
    }

    fn own_resource() -> TestFooResource {
        TestFooResource::new(TestFooResourceImpl)
    }

    fn forwarded_resource(
        res: test_bar::TestBarResource,
    ) -> test_bar::TestBarResource {
        res
    }

    fn get_current_time() -> Datetime {
        wall_clock::now()
    }

    /// Updates the component's state by adding the given value to the total.
    fn add(value: u64) {
        STATE.with_borrow_mut(|state| state.total += value);
    }

    /// Returns the current total.
    fn get() -> u64 {
        // Call code from shared lib
        // println!("{}", example_common_function());

        STATE.with_borrow(|state| state.total)
    }
}

bindings::export!(Component with_types_in bindings);
