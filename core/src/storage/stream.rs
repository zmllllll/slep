use crate::check::Constraint;

use super::*;

#[derive(Clone, Debug)]
pub struct Stream {
    // stream 设置
    pub read_level: Level,
    pub write_level: Level,
}

impl Stream {
    pub fn new() -> Stream {
        Self {
            read_level: constant::MEMBER,
            write_level: constant::MEMBER,
        }
    }

    pub fn update_read_level(&mut self, read_level: Level) {
        self.read_level = read_level;
    }

    pub fn update_write_level(&mut self, write_level: Level) {
        self.write_level = write_level;
    }
}

impl Default for Stream {
    fn default() -> Self {
        Self::new()
    }
}

// impl collect::Collect for (group::Group, Option<Stream>) {
//     type Condition = Option<Stream>;

//     fn collect_all(&self) -> group::Members {
//         self.0.collect_all()
//     }

//     type Container = group::Group;

//     fn gen_container(&self) -> Self::Container {
//         self.0
//     }

//     fn gen_condition(&self) -> Self::Condition {
//         self.1
//     }
// }

impl check::Check<i16> for Level {
    type PassItem = bool;
    fn check_level(&self, constraint: check::Constraint<i16>) -> Self::PassItem {
        match constraint {
            Constraint::Range(com) => match com {
                check::Compare::Le(u) => self.le(&u.0),
                check::Compare::Lt(u) => self.lt(&u.0),
                check::Compare::Between(ul, ll) => self.le(&ul.0) && self.ge(&ll.0),
                check::Compare::Gt(l) => self.gt(&l.0),
                check::Compare::Ge(l) => self.ge(&l.0),
            },
            Constraint::Discrete(multi) => multi.contains(self),
        }
    }
}
