use crate::client::Method;
use crate::methods::condenser::ns;
use crate::methods::utils::{RewardFund, RewardFundType};

pub struct GetRewardFund {
    pub reward_fund_type: RewardFundType,
}

impl GetRewardFund {
    pub fn new(reward_fund_type: RewardFundType) -> Self {
        Self { reward_fund_type }
    }
}

impl Method for GetRewardFund {
    type Params = [RewardFundType; 1];
    type Result = RewardFund;
    const NAME: &'static str = ns!("get_reward_fund");

    fn params(self) -> Self::Params { [self.reward_fund_type] }
}