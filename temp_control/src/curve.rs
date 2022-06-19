

struct CurvePt {
    t: f32
}

struct CurveSegment {
    start: f32,
    end: f32
}

enum Curve {
    Linear(CurveSegment),
    Quadratic(CurveSegment),
    SteppedRamp(Vec<CurveSegment>)
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
