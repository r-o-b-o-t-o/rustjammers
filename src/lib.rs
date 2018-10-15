mod agent;
mod player;
mod frisbee;
mod vector2;
mod game_state;

#[cfg(test)]
mod tests {
    #[test]
    fn multiply_vector() {
        use vector2::Vector2;
        let q: Vector2 = Vector2::new(4.0, 8.0);
        let mut v: Vector2 = Vector2::new(2.0, 4.0);
        v *= 2.0;
        assert_eq!(q, v);
    }
}
