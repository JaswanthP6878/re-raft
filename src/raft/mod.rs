
use raft_log::Log;
use tonic::{transport::Server, Request, Response, Status};


use votes::votes_service_server::{VotesService as VotesServiceServerTrait, VotesServiceServer};
use votes::votes_service_client::{VotesServiceClient};
use votes::{VoteRequest, VoteResponse};

pub mod votes {
    tonic::include_proto!("votes");
}

pub mod raft_log;

pub enum State {
    Leader,
    Candidate,
    Follower
}

pub struct Raft {
    id: u32,
    state : State,
    current_term : u32,
    voted_for:  Option<u32>,

    // The log to be replicated.
    log: Vec<Log>,
}

#[tonic::async_trait]
impl VotesServiceServerTrait for Raft {
    async fn request_vote(&self, vote_request: Request<VoteRequest>) -> Result<Response<VoteResponse>, Status> {
        println!("Got a vote request! {:?}", vote_request);
        let req = vote_request.into_inner();
        let mut response = VoteResponse::default();
        if req.term < self.current_term {
            response.vote_granted = false;
        } else {
            if self.voted_for.is_none_or(|x| x == req.candidate_id) 
                && (req.last_log_index == (self.log.len() - 1) as u32 && req.last_log_term == self.log.last().unwrap().term_number) {
                    response.vote_granted = true;
            }
        }
        Ok(Response::new(response))
    }
}
