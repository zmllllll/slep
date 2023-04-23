pub(crate) trait Check<T> {
    type PassItem;

    fn check_level(&self, constraint: Constraint<T>) -> Self::PassItem;
}

pub(super) struct UpperLimit<T>(pub(super) T);
pub(super) struct LowerLimit<T>(pub(super) T);
pub(crate) enum Constraint<T> {
    Range(Compare<T>),
    #[allow(dead_code)]
    Discrete(Vec<T>),
}

#[allow(dead_code)]
pub(super) enum Compare<T> {
    Le(UpperLimit<T>),
    Lt(UpperLimit<T>),
    Between(LowerLimit<T>, UpperLimit<T>),
    Gt(LowerLimit<T>),
    Ge(LowerLimit<T>),
}
