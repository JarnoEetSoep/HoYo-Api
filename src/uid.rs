use crate::regions::Regions;

pub struct UID(pub String);

impl UID {
    pub fn get_region(&self) -> Regions {
        if ['1','2','5'].into_iter().any(|v| v == self.0.chars().next().unwrap()) {
            Regions::CN
        } else {
            Regions::OS
        }
    }

    pub fn get_server(&self) -> String {
        String::from(match &self.0.chars().next().unwrap() {
            '1' => "cn_gf01",
            '2' => "cn_gf01",
            '5' => "cn_qd01",
            '6' => "os_usa",
            '7' => "os_euro",
            '8' => "os_asia",
            '9' => "os_cht",
            _ => panic!("Invalid uid")
        })
    }
}