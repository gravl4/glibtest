#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

#[allow(dead_code)] // отключаем предупреждение о неиспользуемом коде
pub mod glib {
    pub fn m6_lib_test() {
        println!("LIB FROM GITHUB")
    }

    pub fn m2_lib_test() {
        println!("LIB FROM2 GITHUB")
    }
}
