use estoque::{cadastrar_produtos, criar_tbl_produtos, conectar_banco};

fn main() {
    if let Err(err) = executar_inicializacao() {
        eprintln!("Erro ao inicializar o estoque: {err}");
        std::process::exit(1);
    }
}

fn executar_inicializacao() -> Result<(), Box<dyn std::error::Error>> {
    conectar_banco()?;
    criar_tbl_produtos()?;
    cadastrar_produtos("banana", "12345", 1.0, 2.0)?;

    println!("Estoque inicializado com sucesso!");
    Ok(())
}
