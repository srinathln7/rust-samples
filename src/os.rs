use std::cmp::Ordering;

// Enums are a natural way to express mutually exclusive but related possibilities
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
// Assume we have three priorities based solely on the current State. Any Sleeping process should be the highest priority for execution,
// followed by Stopped processes and then the running process.
pub enum State {
    Running = 3,  // 0 by default
    Stopped = 2,  // 1 by default
    Sleeping = 1, // 2 by default
}

pub fn stop_and_schedule_another_process() {
    println!("stopping and scheduling another process");
}

pub fn assign_to_available_cpu_core() {
    println!("assigning to available cpu core");
}

pub fn check_if_data_ready_and_wake_if_so() {
    println!("check if data is ready and wakes if so");
}

pub fn manage_process(curr_state: State) {
    match curr_state {
        State::Running => stop_and_schedule_another_process(),
        State::Stopped => assign_to_available_cpu_core(),
        State::Sleeping => check_if_data_ready_and_wake_if_so(),
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum StopKind {
    Mandatory, // Linux SIGSTOP
    Ignorable, // Linux SIGSTP
}

// In-memory size of an enum is determined by its largest variant.
//An instance of the Running variant is the same size as an instance of Sleeping variant,
// despite the latter holding more information.
pub enum DetailedState {
    // An enum variant can be like a unit struct without fields or data types
    Running,

    // An enum variant can be like a classic struct with named fields and their data types
    Stopped { reason: StopKind },
    Sleeping { start_time: u64 },
}

#[test]
fn test_detailed_stop_match() {
    let state = DetailedState::Stopped {
        reason: StopKind::Mandatory,
    };
    match state {
        DetailedState::Stopped { reason } => {
            assert_eq!(reason, StopKind::Mandatory);
        }
        _ => unreachable!(),
    }
}

// Use Generic typing: The Rust compiler implements generics via monomorphization.
// Hence generics have no runtime cost
#[derive(Debug)]
pub struct Proc<T> {
    pid: T,           // Process ID (unsigned integer)
    state: State,     // Current state (enum)
    children: Vec<T>, // Child IDs (dynamic list)
}

// Traits are powerful: n implementing a trait manually, we've changed not only how Proc structs
// should be ordered for sorting but also what it means for two Proc structs to be equal.
impl<T> Ord for Proc<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.state.cmp(&other.state)
    }
}

impl<T> PartialOrd for Proc<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<T> PartialEq for Proc<T> {
    fn eq(&self, other: &Self) -> bool {
        self.state == other.state
    }
}

impl<T> Eq for Proc<T> {}

impl<T> Proc<T> {
    /// Associated function (constructor)
    pub fn new(pid: T) -> Self {
        Proc {
            pid,
            state: State::Stopped,
            children: Vec::new(),
        }
    }

    /// Method (takes self, mutable setter in this case)
    pub fn set_state(&mut self, new_state: State) {
        self.state = new_state;
    }
    // ...more methods/functions here
}
