mod task;
mod errors;


pub struct Scheduler
{
    total_num_of_tasks : usize,
    tasks              : Vec<task::Task>,
}


impl Scheduler
{
    pub fn new() -> Self
    {
        Scheduler {
            total_num_of_tasks : 0_usize,
            tasks              : vec![]
        }
    }
    
    pub fn push(&self,  description: String)
    {
        task::Task::new(self.total_num_of_tasks, description);
    }
    
    pub fn peek(&self) -> Option<&task::Task>
    {
        self.tasks.last()
    }
    
    pub fn pop(&mut self) -> Option<task::Task>
    {
        self.tasks.pop()
    }
}
