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
