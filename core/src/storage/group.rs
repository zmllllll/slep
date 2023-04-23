use super::*;
use crate::check::{Check, Constraint};

#[derive(Debug, Default)]
pub struct Groups(pub HashMap<i64, Group>);

impl Groups {}

impl Deref for Groups {
    type Target = HashMap<i64, Group>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Groups {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[derive(Clone, Debug, Default)]
pub struct Group {
    // 组信息
    pub group_info: GroupInfo,
    // 组成员列表    key: uid(成员id)   value: GroupMember(组成员)
    pub group_members: HashMap<i64, GroupMember>,
    // streams列表  key: stream_name   value: StreamSettings(stream设置)
    pub streams: HashMap<String, stream::Stream>,
}

impl Group {
    pub fn _get_info(&self) -> &GroupInfo {
        &self.group_info
    }
    pub fn get_members(&self) -> &HashMap<i64, GroupMember> {
        &self.group_members
    }

    pub fn get_stream(&self) -> &HashMap<String, stream::Stream> {
        &self.streams
    }
}

impl collect::Collect for Group {
    /// Collect all members who are in the group
    fn collect_all(&self) -> Members {
        self.get_members().keys().cloned().collect()
    }

    type Condition = bool;

    type Container = std::collections::hash_set::IntoIter<i64>;

    fn gen_container(&self) -> Self::Container {
        self.collect_all().into_iter()
    }

    fn gen_condition(&self) -> Self::Condition {
        true
    }
}

pub type Members = HashSet<i64>;
impl Group {
    /// Returns true if the group contains the user for the specified uid.
    pub fn _contains_self(&self, myself: i64) -> bool {
        self.get_members().contains_key(&myself)
    }

    // Collect all reviewers who are in the group
    // pub fn filter_collect(&self) -> Members {
    //     self.group_members
    //         .iter()
    //         .filter_map(|member| {
    //             Member::check_level(&member, group::Constraint::Discrete(vec![REVIEWER]))
    //         })
    //         .collect()
    // }
}

// 组信息
#[derive(Clone, Debug, Default)]
pub struct GroupInfo {
    // 父组id
    pid: Option<i64>,
    // 组名
    name: String,
}

impl GroupInfo {
    pub fn update_pid(&mut self, pid: Option<i64>) {
        self.pid = pid;
    }

    pub fn update_name(&mut self, name: String) {
        self.name = name;
    }
}

/// GroupMember 组成员
///
/// level 用户等级
#[derive(Clone, Debug, Default)]
pub struct GroupMember {
    level: i16,
}

impl GroupMember {
    pub fn update(&mut self, level: i16) {
        self.level = level;
    }
}

type Member<'a> = (&'a i64, &'a GroupMember);

impl Check<i16> for Member<'_> {
    type PassItem = Option<i64>;

    fn check_level(&self, constraint: Constraint<i16>) -> Self::PassItem {
        match constraint {
            Constraint::Range(com) => match com {
                check::Compare::Le(u) => Some(self.0).filter(|_| self.1.level.le(&u.0)).cloned(),
                check::Compare::Lt(u) => Some(self.0).filter(|_| self.1.level.lt(&u.0)).cloned(),
                check::Compare::Between(ll, ul) => Some(self.0)
                    .filter(|_| self.1.level.le(&ul.0) && self.1.level.ge(&ll.0))
                    .cloned(),
                check::Compare::Gt(l) => Some(self.0).filter(|_| self.1.level.gt(&l.0)).cloned(),
                check::Compare::Ge(l) => Some(self.0).filter(|_| self.1.level.ge(&l.0)).cloned(),
            },
            Constraint::Discrete(multi) => Some(self.0)
                .filter(|_| multi.contains(&self.1.level))
                .cloned(),
        }
    }
}

impl Check<i16> for GroupMember {
    type PassItem = bool;

    fn check_level(&self, constraint: Constraint<i16>) -> Self::PassItem {
        match constraint {
            Constraint::Range(com) => match com {
                check::Compare::Le(u) => self.level.le(&u.0),
                check::Compare::Lt(u) => self.level.lt(&u.0),
                check::Compare::Between(ll, ul) => self.level.le(&ul.0) && self.level.ge(&ll.0),
                check::Compare::Gt(l) => self.level.gt(&l.0),
                check::Compare::Ge(l) => self.level.ge(&l.0),
            },
            Constraint::Discrete(multi) => multi.contains(&self.level),
        }
    }
}
