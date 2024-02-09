#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Status {
    UNKNOWN,
    RED,
    YELLOW,
    GREEN,
}

impl PartialOrd for Status {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        use std::cmp::Ordering;

        match (self, other) {
            (Status::GREEN, Status::GREEN)
            | (Status::YELLOW, Status::YELLOW)
            | (Status::RED, Status::RED)
            | (Status::UNKNOWN, Status::UNKNOWN) => Some(Ordering::Equal),

            (Status::GREEN, _)
            | (Status::YELLOW, Status::RED)
            | (Status::YELLOW, Status::UNKNOWN)
            | (Status::RED, Status::UNKNOWN) => Some(Ordering::Greater),

            _ => Some(Ordering::Less),
        }
    }
}

impl Ord for Status {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}
