// TODO: Implement `TryFrom<String>` and `TryFrom<&str>` for `Status`.
//  The parsing should be case-insensitive.

#[derive(Debug, PartialEq, Clone)]
enum Status {
    ToDo,
    InProgress,
    Done,
}

impl TryFrom<&str> for Status {
    type Error = ();
    fn try_from(s: &str) -> Result<Status, Self::Error> {
        let s = s.to_lowercase();
        if s == "todo" {
            Ok(Self::ToDo)
        } else if s == "inprogress" {
            Ok(Self::InProgress)
        } else if s == "done" {
            Ok(Self::Done)
        } else {
            Err(())
        }
    }
}

impl TryFrom<String> for Status {
    type Error = ();
    fn try_from(s: String) -> Result<Status, Self::Error> {
        let s = s.to_lowercase();
        if s == "todo" {
            Ok(Self::ToDo)
        } else if s == "inprogress" {
            Ok(Self::InProgress)
        } else if s == "done" {
            Ok(Self::Done)
        } else {
            Err(())
        }
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
        let status = Status::try_from("todo").unwrap();
        assert_eq!(status, Status::ToDo);

        let status = Status::try_from("inprogress").unwrap();
        assert_eq!(status, Status::InProgress);

        let status = Status::try_from("done").unwrap();
        assert_eq!(status, Status::Done);
    }
}
