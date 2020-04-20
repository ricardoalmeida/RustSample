pub mod greetings{
    pub mod english;
    pub mod french{
        pub fn hello() -> String { "bonjour".to_string() }
        pub fn goodbye() -> String { "au revoir".to_string() }
    }
}