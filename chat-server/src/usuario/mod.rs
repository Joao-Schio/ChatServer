use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::hash::{Hash, Hasher};
#[derive(Debug)]
pub struct Usuario{
    nome    : String,
    online  : AtomicBool
}


impl Usuario{
    pub fn new(nome : String) -> Self{
        Self { nome: nome, online: AtomicBool::new(true) }
    }

    #[inline]
    pub fn get_nome(&self) -> &String{
        return &self.nome;
    }

    #[inline]
    pub fn esta_online(&self) -> bool {
        self.online.load(Ordering::Relaxed)
    }    
    #[inline]
    pub fn set_online(&self, v: bool) {
        self.online.store(v, Ordering::Relaxed);
    }
}

impl PartialEq for Usuario {
    fn eq(&self, other: &Self) -> bool {
        self.nome == other.nome
    }
}

impl Eq for Usuario {}

impl Hash for Usuario {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.nome.hash(state);
    }
}

pub type RefUsuario = Arc<Usuario>;