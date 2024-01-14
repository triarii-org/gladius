use bernardo::config::config::ConfigRef;
use bernardo::w7e::handler::Handler;
use bernardo::w7e::handler_load_error::HandlerLoadError;
use bernardo::w7e::navcomp_group::NavCompTickSender;
use bernardo::w7e::project_scope::ProjectScope;

pub trait NavCompLoader: Send + Sync + 'static {
    fn load_handler(
        &self,
        config: &ConfigRef,
        project_scope: &ProjectScope,
        navcomp_tick_sender: NavCompTickSender,
    ) -> Result<Box<dyn Handler>, HandlerLoadError>;
}
