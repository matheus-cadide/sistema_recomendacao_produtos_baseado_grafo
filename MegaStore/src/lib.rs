use rusqlite::{Connection, params, OptionalExtension};
use std::collections::{HashMap, HashSet, VecDeque};


pub fn recomendar_produtos(
    grafo: &HashMap<String, Vec<String>>,
    codigo_inicial: &str,
) -> Vec<String> {
    let mut fila: VecDeque<(String, usize)> = VecDeque::new();
    let mut visitados: HashSet<String> = HashSet::new();
    let mut recomendacoes: Vec<String> = Vec::new();

    // O produto inicial já foi visitado
    visitados.insert(codigo_inicial.to_string());

    // O segundo elemento da tupla representa a distância
    fila.push_back((codigo_inicial.to_string(), 0));

    // Limita a busca a duas conexões de distância
    let distancia_maxima = 2;

    while let Some((codigo_atual, distancia)) = fila.pop_front() {
        // Não expande os vértices que chegaram ao limite
        if distancia >= distancia_maxima {
            continue;
        }

        if let Some(vizinhos) = grafo.get(&codigo_atual) {
            for vizinho in vizinhos {
                // Só processa produtos ainda não visitados
                if visitados.insert(vizinho.clone()) {
                    recomendacoes.push(vizinho.clone());

                    fila.push_back((
                        vizinho.clone(),
                        distancia + 1,
                    ));
                }
            }
        }
    }

    recomendacoes
}

//fazer conexão com o banco de dados

pub fn conectar_banco() -> rusqlite::Result<Connection>{
    let conexao = Connection ::open("estoque.db")?;    
    
    Ok(conexao)
}

//criar tabela de produtos
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

//cadastrar produtos


pub fn cadastrar_produtos(nome_produto: &str, codigo_produto: &str, preco_produto: f64) -> rusqlite::Result<()>{
    let conexao = conectar_banco()?;
    conexao.execute("insert into tbl_estoque (nome_produto, codigo_produto, preco_produto) values(?, ?, ?)", params![nome_produto, codigo_produto,  preco_produto,])?;

    Ok(())
}

//visualizar produtos cadastrados

pub fn visualizar_produtos() -> rusqlite::Result<()> {
    let conexao = conectar_banco()?;
    let mut stmt =conexao.prepare("select nome_produto, codigo_produto, preco_produto from tbl_estoque")?;
    // Executa a consulta e obtém os resultados
    let produtos = stmt.query_map([], |linha| {
        Ok((
            linha.get::<_, String>(0)?,
            linha.get::<_, String>(1)?,
            linha.get::<_, f64>(2)?,
            
        ))
    })?;

    // Itera sobre os resultados e imprime os produtos
    for produto in produtos {
        let (nome,codigo, preco) = produto?;
        println!("Nome: {nome}, Código: {codigo}, Preço: {preco}");
    }
    Ok(())
}

//procurar produto pelo código

pub fn procurar_produto_codigo(codigo_produto: &str) -> rusqlite::Result<Option<(String, String, f64)>> {
    
    let conexao = conectar_banco()?;
    //Consulta SQL para buscar o produto pelo código
    let mut stmt = conexao.prepare(
        "SELECT nome_produto, codigo_produto, preco_produto
         FROM tbl_estoque
         WHERE codigo_produto = ?"
    )?;
    // Executa a consulta e obtém o resultado
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

    // Retorna o resultado da consulta

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

pub fn remover_produto(codigo_produto: &str) -> rusqlite::Result<()> {
    let conexao = conectar_banco()?;

    let resultado = conexao.execute(
        "DELETE FROM tbl_estoque WHERE codigo_produto = ?",
        params![codigo_produto],
    )?;

    match resultado {
        0 => {
            println!("Produto não encontrado.");
        }
        _ => {
            println!("Produto removido com sucesso.");
        }
    }

    Ok(())
}

// Banco de dados para testes
pub struct Estoque {
    conexao: Connection,
}

impl Estoque {
    // Abre o banco real do programa.
    pub fn abrir() -> rusqlite::Result<Self> {
        let conexao = Connection::open("estoque.db")?;
        Ok(Self { conexao })
    }

    // Cria um banco exclusivo em memória para os testes.
    pub fn em_memoria() -> rusqlite::Result<Self> {
        let conexao = Connection::open_in_memory()?;
        let estoque = Self { conexao };
        estoque.criar_tabela()?;
        Ok(estoque)
    }

    pub fn criar_tabela(&self) -> rusqlite::Result<()> {
        self.conexao.execute(
            "CREATE TABLE IF NOT EXISTS tbl_estoque (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                nome_produto TEXT NOT NULL,
                codigo_produto TEXT NOT NULL UNIQUE,
                preco_produto REAL NOT NULL
            )",
            [],
        )?;

        Ok(())
    }

    pub fn cadastrar_produtos(
        &self,
        nome: &str,
        codigo: &str,
        preco: f64,
    ) -> rusqlite::Result<()> {
        self.conexao.execute(
            "INSERT INTO tbl_estoque
             (nome_produto, codigo_produto, preco_produto)
             VALUES (?, ?, ?)",
            params![nome, codigo, preco],
        )?;

        Ok(())
    }

    pub fn procurar_produto_codigo(
        &self,
        codigo: &str,
    ) -> rusqlite::Result<Option<(String, String, f64)>> {
        self.conexao
            .query_row(
                "SELECT nome_produto, codigo_produto, preco_produto
                 FROM tbl_estoque
                 WHERE codigo_produto = ?",
                params![codigo],
                |linha| {
                    Ok((
                        linha.get(0)?,
                        linha.get(1)?,
                        linha.get(2)?,
                    ))
                },
            )
            .optional()
    }

    pub fn remover_produto(&self, codigo: &str) -> rusqlite::Result<bool> {
        let quantidade = self.conexao.execute(
            "DELETE FROM tbl_estoque WHERE codigo_produto = ?",
            params![codigo],
        )?;

        Ok(quantidade > 0)
    }
}
