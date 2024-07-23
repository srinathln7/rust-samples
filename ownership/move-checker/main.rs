#[derive(Debug)]
pub enum State {
    Running,
    Stopped,
    Sleeping,
}

#[derive(Debug)]
pub struct Proc {
    // &str, is an immutable string reference. &'static str, means names are string literals whose lifetime ('static) is
    // the entire run of the program, including before and after the main function (during which your OS does setup and teardown tasks).
    name: &'static str,

    state: State,
    children: Vec<Proc>,
}

impl Proc {
    pub fn new(name: &'static str, state: State, children: Vec<Proc>) -> Self {
        Proc {
            name: name,
            state: state,
            children: children,
        }
    }
}

fn main() {
    // Build process tree using 3 "moves" (more info soon):
    //
    // init
    //  |- cron
    //  |- rsyslogd
    //      |- bash
    //
    // Run "pstree -n -g" (in container) to see your OS's real process tree!

    // None of our values own their string names ("init", "cron", "rsyslogd", and "bash").
    // They're just borrowing a reference (&) to something that lives "forever" (until process termination) and doesn't need to be freed.
    // These strings are baked into the compiled binary.

    // Alloc bash
    let bash = Proc::new("bash", State::Running, Vec::new());

    // Error: can't print an ownerless value!
    // dbg!(bash);

    // Alloc rsyslogd, 1st move: bash -> rsyslogd
    let rsyslogd = Proc::new("rsyslogd", State::Running, vec![bash]);

    // Alloc cron
    let cron = Proc::new("cron", State::Sleeping, Vec::new());

    // Alloc init, 2nd and 3rd moves: cron -> init, rsyslogd -> init
    let init = Proc::new("init", State::Running, vec![cron, rsyslogd]);

    // Print serialized tree to see ownership hierarchy
    dbg!(init);

    let x = "Hello Rust!!!".to_string();

    // let y = x; didn't create a copy of the heap-allocated String. That'd be expensive for long strings,
    // and thus requires an explicit call to a data duplication function (e.g. let y = x.clone();). Rust prefers performance.
    // Instead, we performed a move - a cheap transfer of ownership: a move of a fat pointer from x into y (stack variables)

    let y = x; // x moved into y. y now owns String value "Hello!"

    // This works
    println!("Owned string: {y}");

    // This would cause a compile-time error, x is "gone", its value moved!
    //println!("Owned string: {x}");
}

// End of scope, `y` and `init` is dropped here.

impl Drop for Proc {
    fn drop(&mut self) {
        println!("De-alloc-ing \'{}\' Proc @ {:p}", self.name, self);
    }
}

// The 48-byte size we assert includes only a fat pointer (tuple of memory address, total capacity, and current length) to the children heap data.
// Similarly, the name field is a pointer to a string hardcoded into read-only memory. Pointers are just memory addresses with machine specific widths,
// hence our caveat about the size being for a "64-bit machine".
#[test]
fn test_size() {
    assert_eq!(core::mem::size_of::<Proc>(), 48);
}
