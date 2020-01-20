use serde::Deserialize;

mod random;

#[derive(Deserialize)]
pub(crate) enum Strategy {
    BinaryGender,
    Date,
    Characters,
    Name,
    Prose,
    PhoneNr,
    OneOf(Vec<String>),
}

impl Strategy {
    // Generate a valid and anonymous value for the field.
    pub(crate) fn generate(&self) -> String {
        use Strategy::*;
        match self {
            BinaryGender => if rand::random() { "M".to_string() } else { "V".to_string() },
            Date => random::date(),
            Characters => {
                let mut initials = String::new();
                loop {
                    let c = random::char().to_ascii_uppercase();
                    initials.push(c);
                    if rand::random() {
                        break;
                    }
                }
                initials
            },
            Name => format!("{} {}", random::given_name(), random::surname()),
            Prose => random::prose(),
            PhoneNr => random::phone_nr(),
            OneOf(list) => random::element(list),
        }
    }
}
