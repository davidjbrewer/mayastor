use crate::core::Reactor;

/// Dump detailed diagnostic information for the frozen reactor.
/// As of now print only basic information about the reactor,
/// in the future might print stack traces too.
pub(crate) fn diagnose_reactor(reactor: &Reactor) {
    info!(
        core=reactor.core(),
        state=%reactor.get_state(),
        "Reactor is frozen"
    );
}
