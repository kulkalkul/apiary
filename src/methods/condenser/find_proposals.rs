use crate::client::Method;
use crate::methods::condenser::ns;
use crate::methods::utils::{ObjectId, Proposal, ProposalObjectId};

#[derive(Debug)]
pub struct FindProposals {
    pub proposal_object_ids: Vec<ProposalObjectId>,
}

impl FindProposals {
    pub fn new(proposal_object_ids: &[u32]) -> Self {
        Self {
            proposal_object_ids: ObjectId::ids_from_slice(proposal_object_ids),
        }
    }
}

impl Method for FindProposals {
    type Params = [Vec<ProposalObjectId>; 1];
    type Result = Vec<Proposal>;
    const NAME: &'static str = ns!("find_proposals");

    fn params(self) -> Self::Params { [self.proposal_object_ids] }
}