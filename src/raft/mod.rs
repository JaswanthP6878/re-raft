mod timer;
use std::sync::{Arc};

use rand::prelude::*;

use tokio::sync::Mutex;

use raft_log::Log;
use tonic::{transport::Server, Request, Response, Status};


use votes::votes_service_server::{VotesService as VotesServiceServerTrait, VotesServiceServer};
use votes::votes_service_client::{VotesServiceClient};
use votes::{VoteRequest, VoteResponse};

pub mod votes {
    tonic::include_proto!("votes");
}

pub mod raft_log;


#[derive(PartialEq)]
pub enum State {
    Leader,
    Candidate,
    Follower
}
pub struct Raft {
    state: State,
    id: u32,
    current_term : u32,
    voted_for:  Option<u32>,
    // The log to be replicated.
    log: Vec<Log>,
}

impl Default for Raft {
    fn default() -> Self {

        let mut rng= rand::rng();
        Self {
            state: State::Follower,
            id: rng.gen_range(..10),
            current_term: 1,
            voted_for: None,
            log: vec![]
        }
    }

}

impl Raft {
    fn new(id: u32) -> Self {
        Self {
            state: State::Follower,
            id,
            current_term: 1,
            voted_for: None,
            log: vec![]
        }
    }
}

// Raft Service that is gonna run in the background;
pub struct RaftService {
    raft : Arc<Mutex<Raft>>,
    peers: Vec<String>
}

impl RaftService {
    fn new(id: u32, peers: Vec<String>) -> Self {
        Self {
            raft: Arc::new(Mutex::new(Raft::new(id))),
            peers
        }
    }

    async fn run() {
        // start election timer, if election timer goes off send rpc to all the other service
        //  start the request vote rpc serer
        loop {
            
        }
    }
}


// for the Raft trait to recive vote requests
#[tonic::async_trait]
impl VotesServiceServerTrait for RaftService {
    async fn request_vote(&self, vote_request: Request<VoteRequest>) -> Result<Response<VoteResponse>, Status> {
        println!("Got a vote request! {:?}", vote_request);
        let req = vote_request.into_inner();
        let mut response = VoteResponse::default();
        let mut raft = self.raft.lock().await;
        response.term = raft.current_term;
        if req.term < raft.current_term {
            response.vote_granted = false;
        } else {
            if raft.voted_for.is_none_or(|x| x == req.candidate_id) 
                && (req.last_log_index == (raft.log.len() - 1) as u32 && req.last_log_term == raft.log.last().unwrap().term_number) {
                    response.vote_granted = true;
                    raft.state = State::Follower;
            }
        }
        Ok(Response::new(response))
    }
}
