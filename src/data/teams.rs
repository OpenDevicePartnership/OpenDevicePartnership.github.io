//! Team rosters surfaced on the community and per-team pages.
//!
//! Each function returns an owned `Vec<TeamMember>` because that is
//! what the existing `<TeamGrid>` component consumes. Contributors
//! who want to add or remove members only need to edit this file.

#[derive(Clone, Debug)]
pub struct TeamMember {
    pub first_name: &'static str,
    pub last_name: &'static str,
    pub role: &'static str,
    pub github_username: &'static str,
    pub github_url: &'static str,
    pub image_url: &'static str,
}

pub fn steering_committee() -> Vec<TeamMember> {
    vec![
        TeamMember {
            first_name: "Karan",
            last_name: "Dhillon",
            role: "Member",
            github_username: "dhillonk",
            github_url: "https://github.com/dhillonk",
            image_url: "https://github.com/dhillonk.png?size=200",
        },
        TeamMember {
            first_name: "Jerry",
            last_name: "Xie",
            role: "Member",
            github_username: "jerrysxie",
            github_url: "https://github.com/jerrysxie",
            image_url: "https://github.com/jerrysxie.png?size=200",
        },
        TeamMember {
            first_name: "Michael",
            last_name: "Kubacki",
            role: "Member",
            github_username: "makubacki",
            github_url: "https://github.com/makubacki",
            image_url: "https://github.com/makubacki.png?size=200",
        },
    ]
}

pub fn patina_team() -> Vec<TeamMember> {
    vec![
        TeamMember {
            first_name: "Michael",
            last_name: "Kubacki",
            role: "Project Lead and System Management Mode (MM)",
            github_username: "makubacki",
            github_url: "https://github.com/makubacki",
            image_url: "https://github.com/makubacki.png?size=200",
        },
        TeamMember {
            first_name: "Ron",
            last_name: "Gurr",
            role: "Partner Engagement Lead",
            github_username: "rogurr",
            github_url: "https://github.com/rogurr",
            image_url: "https://github.com/rogurr.png?size=200",
        },
        TeamMember {
            first_name: "Oliver",
            last_name: "Smith-Denny",
            role: "Memory Protections and Paging",
            github_username: "os-d",
            github_url: "https://github.com/os-d",
            image_url: "https://github.com/os-d.png?size=200",
        },
        TeamMember {
            first_name: "Vineel",
            last_name: "Kovvuri",
            role: "CPU",
            github_username: "vineelko",
            github_url: "https://github.com/vineelko",
            image_url: "https://github.com/vineelko.png?size=200",
        },
        TeamMember {
            first_name: "Sherry",
            last_name: "Fan",
            role: "Patina Readiness Tool",
            github_username: "berlin-with0ut-return",
            github_url: "https://github.com/berlin-with0ut-return",
            image_url: "https://github.com/berlin-with0ut-return.png?size=200",
        },
        TeamMember {
            first_name: "Chris",
            last_name: "Fernald",
            role: "Debugger",
            github_username: "cfernald",
            github_url: "https://github.com/cfernald",
            image_url: "https://github.com/cfernald.png?size=200",
        },
        TeamMember {
            first_name: "John",
            last_name: "Schock",
            role: "Memory Allocator",
            github_username: "joschock",
            github_url: "https://github.com/joschock",
            image_url: "https://github.com/joschock.png?size=200",
        },
        TeamMember {
            first_name: "Joey",
            last_name: "Vagedes",
            role: "Component Infrastructure",
            github_username: "Javagedes",
            github_url: "https://github.com/Javagedes",
            image_url: "https://github.com/Javagedes.png?size=200",
        },
        TeamMember {
            first_name: "Mathieu",
            last_name: "Gravel",
            role: "Performance",
            github_username: "magravel",
            github_url: "https://github.com/magravel",
            image_url: "https://github.com/magravel.png?size=200",
        },
    ]
}

pub fn ec_team() -> Vec<TeamMember> {
    vec![
        TeamMember {
            first_name: "Jerry",
            last_name: "Xie",
            role: "Team leader",
            github_username: "jerrysxie",
            github_url: "https://github.com/jerrysxie",
            image_url: "https://github.com/jerrysxie.png?size=200",
        },
        TeamMember {
            first_name: "Felipe",
            last_name: "Balbi",
            role: "",
            github_username: "felipebalbi",
            github_url: "https://github.com/felipebalbi",
            image_url: "https://github.com/felipebalbi.png?size=200",
        },
        TeamMember {
            first_name: "Robert",
            last_name: "Zieba",
            role: "",
            github_username: "RobertZ2011",
            github_url: "https://github.com/RobertZ2011",
            image_url: "https://github.com/RobertZ2011.png?size=200",
        },
        TeamMember {
            first_name: "Matteo",
            last_name: "Tullo",
            role: "",
            github_username: "tullom",
            github_url: "https://github.com/tullom",
            image_url: "https://github.com/tullom.png?size=200",
        },
        TeamMember {
            first_name: "Kurtis",
            last_name: "Dinelle",
            role: "",
            github_username: "kurtjd",
            github_url: "https://github.com/kurtjd",
            image_url: "https://github.com/kurtjd.png?size=200",
        },
        TeamMember {
            first_name: "Jimi",
            last_name: "Huard",
            role: "",
            github_username: "JamesHuard",
            github_url: "https://github.com/JamesHuard",
            image_url: "https://github.com/JamesHuard.png?size=200",
        },
        TeamMember {
            first_name: "Adam",
            last_name: "Sasine",
            role: "",
            github_username: "asasine",
            github_url: "https://github.com/asasine",
            image_url: "https://github.com/asasine.png?size=200",
        },
        TeamMember {
            first_name: "Billy",
            last_name: "Price",
            role: "",
            github_username: "williampMSFT",
            github_url: "https://github.com/williampMSFT",
            image_url: "https://github.com/williampMSFT.png?size=200",
        },
    ]
}

pub fn ec_services_team() -> Vec<TeamMember> {
    vec![
        TeamMember {
            first_name: "Phil",
            last_name: "Weber",
            role: "Team leader",
            github_username: "philgweber",
            github_url: "https://github.com/philgweber",
            image_url: "https://github.com/philgweber.png?size=200",
        },
        TeamMember {
            first_name: "Dylan",
            last_name: "Knutson",
            role: "Team leader",
            github_username: "dymk",
            github_url: "https://github.com/dymk",
            image_url: "https://github.com/dymk.png?size=200",
        },
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_member_well_formed(name: &str, member: &TeamMember) {
        assert!(!member.first_name.is_empty(), "{name}: first_name must not be empty");
        assert!(!member.last_name.is_empty(), "{name}: last_name must not be empty");
        assert!(
            !member.github_username.is_empty(),
            "{name}: github_username must not be empty"
        );
        assert_eq!(
            member.github_url,
            format!("https://github.com/{}", member.github_username),
            "{name}: github_url must point at the configured username"
        );
        assert_eq!(
            member.image_url,
            format!("https://github.com/{}.png?size=200", member.github_username),
            "{name}: image_url must derive from the configured username"
        );
    }

    fn assert_roster_well_formed(name: &str, roster: &[TeamMember]) {
        assert!(!roster.is_empty(), "{name}: roster must not be empty");
        for member in roster {
            assert_member_well_formed(name, member);
        }
    }

    #[test]
    fn steering_committee_well_formed() {
        assert_roster_well_formed("steering_committee", &steering_committee());
    }

    #[test]
    fn patina_team_well_formed() {
        assert_roster_well_formed("patina_team", &patina_team());
    }

    #[test]
    fn ec_team_well_formed() {
        assert_roster_well_formed("ec_team", &ec_team());
    }

    #[test]
    fn ec_services_team_well_formed() {
        assert_roster_well_formed("ec_services_team", &ec_services_team());
    }
}
