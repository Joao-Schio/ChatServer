pub mod servidor;
pub mod mensagem;
pub mod usuario;
pub mod conversa;
pub mod comando;
pub mod resposta_servidor;

pub fn debug_print<T: std::fmt::Debug>(label: &str, value: &T) {
    println!("[DEBUG] {} = {:#?}", label, value);
}

#[macro_export]
macro_rules! debug_print {
    ($($arg:tt)*) => {
        #[cfg(debug_assertions)]
        {
            $crate::debug_print($($arg)*);
        }
    };
}