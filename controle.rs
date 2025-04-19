use std::io::{self, Write};
fn main() {
    let     idade     =     25    ;
    if     idade     >     18     {        println!        (        "Você é maior de idade!"        )        ;    } else {        println!        (        "Você é menor de idade!"        )        ;    }
    let mut     contador     =     1    ;
    while     contador     <     5     {        println!        (        "Contagem: {}"        ,         contador        )        ;        contador         =         contador         +         1        ;    }
    println!    (    "Loop finalizado!"    )    ;
}
