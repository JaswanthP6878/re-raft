pub mod raft_log;

pub enum State {
    Leader,
    Candidate,
    Follower
}

pub struct Raft {
    Id: i32,
    state : State,
    currentTerm : i32,
    votedFor:  i32,

    // The log to be replicated.
    log: [raft_log::Log],
}