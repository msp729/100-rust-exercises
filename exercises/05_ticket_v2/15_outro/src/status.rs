// TODO: Implement `TryFrom<String>` and `TryFrom<&str>` for the `Status` enum.
//  The parsing should be case-insensitive.

#[derive(Debug, Clone, PartialEq)]
pub enum Status {
    ToDo,
    InProgress,
    Done,
}
#[derive(Debug, Clone, PartialEq)]
pub struct StatusErr;

impl TryFrom<&str> for Status {
    type Error = StatusErr;

    fn try_from(s: &str) -> Result<Status, StatusErr> {
        let s = s.to_lowercase();
        if s == "todo" {
            return Ok(Status::ToDo);
        }
        if s == "inprogress" {
            return Ok(Status::InProgress);
        }
        if s == "done" {
            return Ok(Status::Done);
        }
        Err(StatusErr)
    }
}

impl TryFrom<String> for Status {
    type Error = StatusErr;
    fn try_from(s: String) -> Result<Status, StatusErr> {
        Self::try_from(&s as &str)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::convert::TryFrom;

    #[test]
    fn test_try_from_string() {
        let status = Status::try_from("ToDO".to_string()).unwrap();
        assert_eq!(status, Status::ToDo);

        let status = Status::try_from("inproGress".to_string()).unwrap();
        assert_eq!(status, Status::InProgress);

        let status = Status::try_from("Done".to_string()).unwrap();
        assert_eq!(status, Status::Done);
    }

    #[test]
    fn test_try_from_str() {
        let status = Status::try_from("ToDO").unwrap();
        assert_eq!(status, Status::ToDo);

        let status = Status::try_from("inproGress").unwrap();
        assert_eq!(status, Status::InProgress);

        let status = Status::try_from("Done").unwrap();
        assert_eq!(status, Status::Done);
    }

    #[test]
    fn test_try_from_invalid() {
        let status = Status::try_from("Invalid");
        assert!(status.is_err());
    }
}
