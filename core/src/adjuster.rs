use crate::*;

impl<N> StructureAdjuster<N> for N {
    fn try_adjust(self) -> Result<N, StructureUnmatched> {
        Ok(self)
    }
}

impl<N> StructureAdjuster<N> for Vec<N> {
    fn try_adjust(mut self) -> Result<N, StructureUnmatched> {
        if self.len() > 1 {
            Err(StructureUnmatched(format!(
                "expected exactly one element, but found {} elements",
                self.len()
            )))
        } else {
            self.pop().ok_or_else(|| {
                StructureUnmatched("expected exactly one element, but no element found".to_string())
            })
        }
    }
}

impl<N, const A: usize> StructureAdjuster<[N; A]> for Vec<N> {
    fn try_adjust(self) -> Result<[N; A], StructureUnmatched> {
        self.try_into().map_err(|v: Vec<_>| {
            StructureUnmatched(format!(
                "expected exactly {} elements, but found {} elements",
                A,
                v.len()
            ))
        })
    }
}

impl<N> StructureAdjuster<Option<N>> for Vec<N> {
    fn try_adjust(mut self) -> Result<Option<N>, StructureUnmatched> {
        if self.len() > 1 {
            Err(StructureUnmatched(format!(
                "expected at most one element, but found {} elements",
                self.len()
            )))
        } else {
            Ok(self.pop())
        }
    }
}

#[cfg(test)]
mod test {
    use crate::{StructureAdjuster, StructureUnmatched};

    #[test]
    fn single() {
        assert_eq!("foo".try_adjust(), Ok("foo"));
    }

    #[test]
    fn vec_to_single() {
        assert_eq!(vec!["foo"].try_adjust(), Ok("foo"));
        assert_eq!(
            vec![].try_adjust(),
            err::<&str>("expected exactly one element, but no element found"),
        );
        assert_eq!(
            vec!["foo", "bar"].try_adjust(),
            err::<&str>("expected exactly one element, but found 2 elements"),
        );
    }

    #[test]
    fn vec_to_array() {
        assert_eq!(vec!["foo", "bar"].try_adjust(), Ok(["foo", "bar"]));
        assert_eq!(
            vec!["foo", "var"].try_adjust(),
            err::<[&str; 1]>("expected exactly 1 elements, but found 2 elements"),
        );
        assert_eq!(
            vec!["foo", "var"].try_adjust(),
            err::<[&str; 3]>("expected exactly 3 elements, but found 2 elements"),
        );
    }

    #[test]
    fn vec_to_option() {
        assert_eq!(
            (vec![] as Vec<&str>).try_adjust(),
            Ok(None) as Result<Option<&str>, _>
        );
        assert_eq!(vec!["foo"].try_adjust(), Ok(Some("foo")));
        assert_eq!(
            vec!["foo", "var"].try_adjust(),
            err::<Option<&str>>("expected at most one element, but found 2 elements"),
        );
    }

    fn err<T>(s: &str) -> Result<T, StructureUnmatched> {
        Err(StructureUnmatched(s.to_string()))
    }
}
