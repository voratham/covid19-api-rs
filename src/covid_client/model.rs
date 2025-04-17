use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct CovidCaseDataResponse {
    #[serde(rename = "Data")]
    pub data: CovidCases,
}

pub type CovidCases = Vec<CovidCase>;

pub trait CovidCasesExt {
    fn summary(&self) -> (HashMap<String, i64>, HashMap<String, i64>);
    fn total_case(&self) -> i64;
}

impl CovidCasesExt for CovidCases {
    fn total_case(&self) -> i64 {
        self.len() as i64
    }

    fn summary(&self) -> (HashMap<String, i64>, HashMap<String, i64>) {
        let mut new_summary_province = HashMap::new();

        let mut new_summary_age_group = HashMap::from([
            ("N/A".to_string(), 0),
            ("0-30".to_string(), 0),
            ("31-60".to_string(), 0),
            ("61+".to_string(), 0),
        ]);

        for case in self.iter() {
            if let Some(province) = &case.province {
                if !new_summary_province.contains_key(province) {
                    new_summary_province.insert(province.clone(), 1);
                } else {
                    let count = new_summary_province.get_mut(province).unwrap();
                    *count += 1;
                }
            }

            let age_group = match case.age {
                Some(age) if age <= 30 => "0-30",
                Some(age) if age <= 60 => "31-60",
                Some(_) => "61+",
                None => "N/A",
            };
            
            *new_summary_age_group.get_mut(age_group).unwrap() += 1;
        }

        return (new_summary_province, new_summary_age_group);
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CovidCase {
    #[serde(rename = "ConfirmDate")]
    pub confirm_date: Option<String>,
    #[serde(rename = "No")]
    pub no: Option<i64>,
    #[serde(rename = "Age")]
    pub age: Option<i64>,
    #[serde(rename = "Gender")]
    pub gender: Option<String>,
    #[serde(rename = "GenderEn")]
    pub gender_en: Option<String>,
    #[serde(rename = "Nation")]
    pub nation: Option<String>,
    #[serde(rename = "NationEn")]
    pub nation_en: Option<String>,
    #[serde(rename = "Province")]
    pub province: Option<String>,
    #[serde(rename = "ProvinceId")]
    pub province_id: Option<i64>,
    #[serde(rename = "District")]
    pub district: Option<String>,
    #[serde(rename = "ProvinceEn")]
    pub province_en: Option<String>,
    #[serde(rename = "StatQuarantine")]
    pub stat_quarantine: Option<i64>,
}
