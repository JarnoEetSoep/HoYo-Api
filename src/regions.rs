pub enum Regions {
    CN,
    OS,
}

impl Regions {
    pub fn get_claim_daily_url(&self) -> &str {
        match self {
            Self::CN => {
                "https://api-takumi.mihoyo.com/event/bbs_sign_reward/?act_id=e202009291139501"
            }
            Self::OS => "https://sg-hk4e-api.hoyolab.com/event/sol/sign?act_id=e202102251931481",
        }
    }
}
