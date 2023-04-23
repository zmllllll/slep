use crate::storage::group::Members;

pub(crate) trait GenCollector {
    type Target: Collect;
    fn gen_collector(&self) -> Option<Self::Target>;
}

impl GenCollector for (&crate::storage::group::Groups, i64) {
    type Target = crate::storage::group::Group;
    fn gen_collector(&self) -> Option<Self::Target> {
        let (groups, gid) = self;
        groups.get(gid).cloned()
    }
}

// impl GenCollector for (&Groups, i64, &String) {
//     type Target = (super::group::Group, Option<super::stream::Stream>);
//     fn gen_collector(&self) -> Option<Self::Target> {
//         let (groups, gid, s) = self;

//         groups.get(gid).map(|g| {
//             let group = g.to_owned();
//             let stream = g.streams.get(*s).cloned();
//             (group, stream)
//         })
//     }
// }

pub(crate) trait CollectMap<C: Collect> {
    fn collect_map<F: Fn(&C) -> Members>(&self, op: F) -> Members;
}

impl<C: Collect> CollectMap<C> for Option<&C> {
    fn collect_map<F: Fn(&C) -> Members>(&self, op: F) -> Members {
        let o = self;
        match o {
            Some(c) => op(c),
            None => Members::new(),
        }
    }
}

impl<C: Collect> CollectMap<C> for Option<C> {
    fn collect_map<F: Fn(&C) -> Members>(&self, op: F) -> Members {
        let o = self;
        match o {
            Some(c) => op(c),
            None => Members::new(),
        }
    }
}

pub(crate) trait Collect {
    type Condition = bool;
    type Container = std::collections::hash_set::IntoIter<i64>;
    fn collect_all(&self) -> Members;

    fn gen_container(&self) -> Self::Container;

    fn gen_condition(&self) -> Self::Condition;
    // fn collect_filter<F: Fn(std::collections::hash_set::IntoIter<i64>) -> Members>(
    fn collect_filter<F: Fn(Self::Container, Self::Condition) -> Members>(&self, op: F) -> Members {
        op(self.gen_container(), self.gen_condition())
    }
}
