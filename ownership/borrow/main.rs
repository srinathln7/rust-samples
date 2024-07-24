//  Using the type &str would have been even better, because then print_str_len could also work for string slices - including those with static lifetimes.
fn print_str_len_move(s: String) {
    println!("\'{}\' is {} bytes long.", s, s.len()); 
}

fn print_str_len_borrow(s: &String) {
    println!("\'{}\' is {} bytes long ", s, s.len()); 
}

#[derive(Debug)]
pub enum State{
    Running,
    Sleeping,
    Stopped, 
}

// Lifetime annotation 'a indicates that Proc must live at least as long as the references it borrows (non-owned children here).
#[derive(Debug)]
pub struct Proc<'a> {
    name: &'static str, 
    state: State,
    children: Vec<&'a Proc<'a>>, 
}

impl<'a> Proc<'a> {
    pub fn new(name: &'static str, state: State, children: Vec<&'a Proc>) -> Self {
        Proc { name: name, state: state, children: children }
    }
}

impl<'a> Drop for Proc<'a> {
    fn drop(&mut self) {
        println!("De-alloc-ing \'{}\' Proc @ {:p}", self.name, self);
    }
}


fn main() {
    let x = "Hello!".to_string();
    print_str_len_move(x); 

    // This would cause a compile-time error, x is "gone", its value moved!
    // println!("Owned string: {x}");

    // Variable shadowing
    let x = "Hello!".to_string(); 
    print_str_len_borrow(&x);

    // No error this time! x still owns the String.
    println!("Owned string: {x}");

     // Mutuable (Exclusive) references: i.e. Simulataneously only one mutuable references can 
     let mut x = "Hello!".to_string(); 

     let r1 = &mut x ;
     r1.pop();
     r1.push_str(", world");
 
     println!("Modified via r1 {}", r1); 
     // End of implicit (no open-close brackets) scope for 1st mutable borrow,
     // b/c not used again in this function

     let r2 = &mut x ;
     r2.push_str("!"); 
     println!("Modified via r2 {}", r2);

    let bash = Proc::new("bash", State::Running, Vec::new());
    let rsyslogd = Proc::new("rsyslogd", State::Running, vec![&bash]);
    dbg!(&bash); 
    let cron = Proc::new("cron", State::Running, vec![]);
    let init = Proc::new("init", State::Running, vec![&rsyslogd, &cron]); 
    dbg!(&cron);
    dbg!(init);   
}