//! Types related to task management

use super::TaskContext;
use crate::task::MAX_SYSCALL_NUM;

#[derive(Copy, Clone)]
/// task control block structure
pub struct TaskControlBlock {
    pub task_status: TaskStatus,
    pub task_cx: TaskContext,
    // LAB1: Add whatever you need about the Task.
    
    pub syscall_times: [u32; MAX_SYSCALL_NUM],
    pub first_run: bool,
    pub time_begin: usize,
    pub time_end: usize,
}

#[derive(Copy, Clone, PartialEq)]
/// task status: UnInit, Ready, Running, Exited
pub enum TaskStatus {
    UnInit,
    Ready,
    Running,
    Exited,
}
