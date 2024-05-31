// async apps require runtime support
// I/O event loop: "driver", drives I/O resources and dispatches I/O events
// Scheduler: used to execute tasks
// Timer: scheduling work to run after certain time
// when you dont need to fine tune, tokio::main will create a runtime under the hood

// runtime has collection of tasks that it needs to schedule
// repeatedly remove task from collection and schedule it via poll
// no guarantee about the order in which tasks are scheduled or that runtime is fair

// current thread runtime
// two FIFO q of tasks that are ready to be scheduled
// 1) global queue
// 2) local queue
// prefer to schedule tasks from the local queue and go to global if local empty
// will go to global if picked from local 31 times in a row, can be configured
// check for I/O or timer when no new tasks to schedule or scheduled 61 tasks in a row, can be configured


// multi thread runtime
// fixed number of worker threads created on startup
// maintains global and local queue for each worker thread
// local queue can have 256 tasks, if more, half them moved to global
// if local and global queue empty, will try to steal work by taking half the tasks from another local queue



#[tokio::main]
async fn main() {
}