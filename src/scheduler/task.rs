pub struct Task
{
    pub id:   usize,
    pub desc: String,
}

impl Task
{
    pub fn new(id: usize, desc: String) -> Self
    {
        Task{id, desc}
    }
}