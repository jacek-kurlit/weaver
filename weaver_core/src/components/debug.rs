pub mod echo {
    pub struct Echo {
        pub message: String,
    }

    impl Echo {
        pub fn execute(&self) -> Result<String, String> {
            Ok(self.message.clone())
        }
    }
}
