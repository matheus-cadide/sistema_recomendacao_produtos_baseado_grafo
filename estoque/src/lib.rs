use rusqlite::{Connection, params};



pub fn conectar_banco() -> rusqlite::Result<Connection>{
    let conexao = Connection ::open("estoque.db")?;    
    
    Ok(conexao)
}
pub fn criar_tbl_produtos() -> rusqlite::Result<()> {
    let conexao = conectar_banco()?;
    conexao.execute("create table if not exists tbl_estoque(
                        id integer primary key autoincrement,
                        nome_produto varchar not null,
                        codigo_produto varchar not null unique,
                        quantidade_produto real not null,
                        preco_produto real not null
                        )", [])?;
    Ok(())
    
    
}
pub fn cadastrar_produtos(nome_produto: &str, codigo_produto: &str, quantidade_produto: f64, preco_produto: f64) -> rusqlite::Result<()>{
    let conexao = conectar_banco()?;
    conexao.execute("insert into tbl_estoque (nome_produto, codigo_produto, quantidade_produto, preco_produto) values(?, ?, ?, ?)", params![nome_produto, codigo_produto, quantidade_produto, preco_produto,])?;

    Ok(())
}

pub fn visualizar_produtos() -> rusqlite::Result<Vec<(i32, String, String, f64, f64)>> {
    let conexao = conectar_banco()?;
    conexao.execute
}
