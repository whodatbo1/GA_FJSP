use crate::schedule::Schedule;

struct Population<'a> {
    pub members: Vec<Schedule<'a>>
}