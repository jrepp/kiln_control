

struct PidState {
    i: f32,
    d,
    u,
    f
}

struct PidInput {
    temp: f32
}

impl PidState {
    fn next(input: PidInput) -> PidState {

    }
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
