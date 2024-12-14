use std::time::Duration;
use actix_web::dev::Server;
use log::{error, info};
use tokio::time::Instant;

use crate::config::ServerConfig;

pub trait WkServer<ServiceConfig>: Sized {
    fn config(&self) -> &ServerConfig<ServiceConfig>;
    fn create_server(&self) -> Result<Server, std::io::Error>;
    fn server_name(&self) -> &str;
    fn failed_report(&mut self, e: std::io::Error, failure_count: u32, start_time: Instant);

    async fn run_with_restart(mut self) -> Result<(), std::io::Error> {
        if !self.config().enable {
            info!("{} is disabled.", self.server_name());
            return Ok(());
        }

        let max_failures = self.config().max_failures; // 最大失敗回数
        let failure_count_period_time = self.config().failure_count_period_time; // 失敗回数をカウントする時間
        let restart_interval = self.config().restart_interval; // 再起動間隔
        let mut failure_count = 0;

        loop {
            let start_time = Instant::now();
            let mut e = std::io::Error::new(std::io::ErrorKind::Other, "Unknown error");

            match self.create_server() {
                Ok(server) => {
                    // サーバー起動に成功した場合はfailure_countを0に戻す
                    failure_count = 0;

                    if let Err(e) = server.await {
                        error!("{} encountered an error: {}", self.server_name(), e);
                        failure_count += 1;
                    }
                }
                Err(e) => {
                    error!("Failed to initialize {}: {}", self.server_name(), e);
                    failure_count += 1;
                }
            }
            error!("If it fails within {} seconds, it will stop in {} more attempts", failure_count_period_time, max_failures - failure_count);
            self.failed_report(e, failure_count, start_time);

            if start_time.elapsed() >= Duration::from_secs(failure_count_period_time as u64) {
                failure_count -= 1;
            } else if failure_count >= max_failures {  // failure_countが最大失敗回数を超えた場合はループを抜ける
                error!("{} has failed to start {} times. Stopping restart attempts.", self.server_name(), max_failures);
                break;
            }

           


            // 再起動しない設定の場合はループを抜ける
            if !self.config().restart_on_panic {
                info!("{} is set to not restart on panic. Exiting...", self.server_name());
                break;
            }

            error!("Restarting {} in {} seconds...", self.server_name(), restart_interval);
            tokio::time::sleep(Duration::from_secs(restart_interval as u64)).await;
        }

        Ok(())
    }
}
