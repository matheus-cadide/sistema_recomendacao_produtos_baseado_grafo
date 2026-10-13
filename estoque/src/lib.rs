use rusqlite::{Connection, params, OptionalExtension};



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
                        preco_produto real not null
                        )", [])?;
    Ok(())
    
    
}
pub fn cadastrar_produtos(nome_produto: &str, codigo_produto: &str, preco_produto: f64) -> rusqlite::Result<()>{
    let conexao = conectar_banco()?;
    conexao.execute("insert into tbl_estoque (nome_produto, codigo_produto, preco_produto) values(?, ?, ?)", params![nome_produto, codigo_produto,  preco_produto,])?;

    Ok(())
}

pub fn visualizar_produtos() -> rusqlite::Result<()> {
    let conexao = conectar_banco()?;
    let mut stmt =conexao.prepare("select nome_produto, codigo_produto, preco_produto from tbl_estoque")?;

    let produtos = stmt.query_map([], |linha| {
        Ok((
            linha.get::<_, String>(0)?,
            linha.get::<_, String>(1)?,
            linha.get::<_, f64>(2)?,
            
        ))
    })?;
    for produto in produtos {
        let (nome,codigo, preco) = produto?;
        println!("Nome: {nome}, Código: {codigo}, Preço: {preco}");
    }
    Ok(())
}

pub fn procurar_produto_codigo(codigo_produto: &str) -> rusqlite::Result<Option<(String, String, f64)>> {
    
    let conexao = conectar_banco()?;

    let mut stmt = conexao.prepare(
        "SELECT nome_produto, codigo_produto, preco_produto
         FROM tbl_estoque
         WHERE codigo_produto = ?"
    )?;

    let resultado = stmt.query_row(
        params![codigo_produto],
        |linha| {
            Ok((
                linha.get::<_, String>(0)?,
                linha.get::<_, String>(1)?,
                linha.get::<_, f64>(2)?,
            ))
        }
    ).optional();

    match resultado {
        Ok(Some(produto)) => {
            Ok(Some(produto))
        }

        Ok(None) => {
            println!("Produto não encontrado.");
            Ok(None)
        }

        Err(erro) => {
            Err(erro)
        }
    }

}


