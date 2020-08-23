#[macro_export]
macro_rules! my_vec {
    ($($x: expr),*) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
            )*
            temp_vec
        }
    };
}

#[cfg(test)]
mod tests {
    #[test]
    fn creates_vector() {
        let v = my_vec![1, 2, 3];
        assert_eq!(*v.get(0).unwrap(), 1);
        assert_eq!(*v.get(1).unwrap(), 2);
        assert_eq!(*v.get(2).unwrap(), 3);
        assert_eq!(v.len(), 3);
    }
}
