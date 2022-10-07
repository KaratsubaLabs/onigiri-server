// Resources that were used in writing this
// - [periodic interuptable task](https://stackoverflow.com/questions/71096626/how-do-i-run-an-asynchronous-task-periodically-and-also-sometimes-on-demand)
// - [function types
// explained](https://medium.com/swlh/understanding-closures-in-rust-21f286ed1759)

use std::{future::Future, time::Duration};

use async_trait::async_trait;
use tokio::{task::JoinHandle, time::sleep};

use crate::client::{Client, ClientBuilder};

#[async_trait]
pub trait System: Send + Sync {
    type Output;

    async fn run(&self, client: Client) -> Self::Output;
}

#[async_trait]
impl<F, R> System for F
where
    F: Fn(Client) -> R + Send + Sync,
    R: Future + Send,
{
    type Output = <R as Future>::Output;
    async fn run(&self, client: Client) -> Self::Output {
        self(client).await
    }
}

pub struct App {
    handles: Vec<JoinHandle<()>>,
    client_builder: ClientBuilder,
}

impl App {
    pub fn new(client_builder: ClientBuilder) -> Self {
        App {
            handles: vec![],
            client_builder,
        }
    }

    /*
    /// Create a system that will infinitely loop
    pub fn add_system<F>(&mut self, system: F) -> Result<(), anyhow::Error>
    where
        F: System + 'static,
    {
        let client = self.client_builder.connect()?;
        let handle: JoinHandle<()> = tokio::spawn(async move {
            loop {
                system.run(client.clone());
            }
        });

        self.handles.push(handle);
        Ok(())
    }
    */

    /// Create a system that will run over an interval
    pub fn add_periodic_system<F>(
        &mut self,
        system: F,
        duration: Duration,
    ) -> Result<(), anyhow::Error>
    where
        F: System + 'static,
    {
        let client = self.client_builder.connect()?;
        let handle: JoinHandle<()> = tokio::spawn(async move {
            loop {
                system.run(client.clone()).await;
                sleep(duration).await;
            }
        });

        self.handles.push(handle);
        Ok(())
    }

    /// Run a system only once
    pub async fn run_once<F>(&self, system: F) -> Result<(), anyhow::Error>
    where
        F: System + 'static,
    {
        let client = self.client_builder.connect()?;
        let handle: JoinHandle<()> = tokio::spawn(async move {
            system.run(client.clone()).await;
        });
        handle.await;

        Ok(())
    }

    // /// Create a system that runs over an interval and also can be interupted to restart
    // pub fn add_periodic_interuptable_system(&mut self) {}

    // pub fn run() {}
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use tokio::time::sleep;

    use super::App;
    use crate::{
        api::LCDDevice,
        client::{Client, ClientBuilder},
    };

    async fn sys(client: Client) {
        println!("hello world");

        let lcd = client
            .device::<LCDDevice>("rf4smke9g4gwz4eajj9o")
            .await
            .expect("could not find device");
        let res = lcd
            .write_line(1, "hello world")
            .await
            .expect("error interfacing with device");
        println!("{:?}", res);
    }

    #[tokio::test]
    async fn periodic() {
        let client = ClientBuilder::new("http://127.0.0.1:8080/v1beta", "API_KEY");
        let mut app = App::new(client);
        app.add_periodic_system(sys, Duration::from_secs(1))
            .unwrap();

        sleep(Duration::from_secs(5)).await;
    }
}
