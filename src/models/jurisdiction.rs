use std::hash::Hash;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum Country {
    USA,
    Canada,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum USState {
    California,
    NewYork,
    // Add more as needed
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum CanadianProvince {
    Ontario,
    BritishColumbia,
    // Add more as needed
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum Jurisdiction {
    Federal(Country),
    USState(USState),
    CanadianProvince(CanadianProvince),
}

impl Jurisdiction {
    pub fn get_country(&self) -> Country {
        match self {
            Jurisdiction::Federal(country) => country.clone(),
            Jurisdiction::USState(_) => Country::USA,
            Jurisdiction::CanadianProvince(_) => Country::Canada,
        }
    }
}
