#[derive(Debug)]
enum ScheduleError
{
    PushingError,
    PeekingError,
}

pub type ScheduleResult = Result<bool, ScheduleError>;
