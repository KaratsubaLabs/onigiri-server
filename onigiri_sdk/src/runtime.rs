// Resources that were used in writing this
// - [periodic interuptable task](https://stackoverflow.com/questions/71096626/how-do-i-run-an-asynchronous-task-periodically-and-also-sometimes-on-demand)
// - [function types
// explained](https://medium.com/swlh/understanding-closures-in-rust-21f286ed1759) 

use std::time::Duration;

use tokio::{task::JoinHandle, time::sleep};

pub type System = fn();

pub struct App {
    handles: Vec<JoinHandle<()>>,
}

impl App {
    pub fn new() -> Self {
        App { handles: vec![] }
    }

    /// Create a system that will infinitely loop
    pub fn add_system(&mut self, system: System) {
        let handle: JoinHandle<()> = tokio::spawn(async move {
            loop {
                system();
            }
        });

        self.handles.push(handle);
    }

    /// Create a system that will run over an interval
    pub fn add_periodic_system(&mut self, system: System, duration: Duration) {
        let handle: JoinHandle<()> = tokio::spawn(async move {
            loop {
                system();
                sleep(duration).await;
            }
        });

        self.handles.push(handle);
    }

    /// Create a system that runs over an interval and also can be interupted to restart
    pub fn add_periodic_interuptable_system(&mut self) {}

    pub fn run() {}
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use tokio::time::sleep;

    use super::App;

    #[tokio::test]
    async fn periodic() {
        let sys = || {
            println!("hello world");
        };

        let mut app = App::new();
        app.add_periodic_system(sys, Duration::from_secs(1));

        sleep(Duration::from_secs(5)).await;
    }
}
