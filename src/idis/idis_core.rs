use super::config::config::Config;

pub struct IdisCore {
    pub config: Config,
    pub db: DB,
    pub analyze: Analyze,
    pub user_session: UserSession,
    pub processor: Processor,
    pub flame_set
}