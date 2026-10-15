use mega_store::{recomendar_produtos, Estoque};
use std::collections::HashMap;

#[test]
fn produto_sem_conexoes_nao_tem_recomendacoes() {
    let grafo: HashMap<String, Vec<String>> = HashMap::new();

    let resultado = recomendar_produtos(&grafo, "100");

    assert!(resultado.is_empty());
}

#[test]
fn recomenda_produto_conectado() {
    let mut grafo: HashMap<String, Vec<String>> = HashMap::new();

    grafo.insert("100".to_string(), vec!["200".to_string()]);

    let resultado = recomendar_produtos(&grafo, "100");

    assert_eq!(resultado, vec!["200".to_string()]);
}

#[test]
fn nao_recomenda_o_produto_inicial() {
    let mut grafo: HashMap<String, Vec<String>> = HashMap::new();

    grafo.insert("100".to_string(), vec!["200".to_string()]);
    grafo.insert("200".to_string(), vec!["100".to_string()]);

    let resultado = recomendar_produtos(&grafo, "100");

    assert!(!resultado.contains(&"100".to_string()));
}

#[test]
fn nao_repete_produto_alcancado_por_dois_caminhos() {
    let mut grafo: HashMap<String, Vec<String>> = HashMap::new();

    grafo.insert(
        "100".to_string(),
        vec!["200".to_string(), "300".to_string()],
    );
    grafo.insert(
        "200".to_string(),
        vec!["100".to_string(), "400".to_string()],
    );
    grafo.insert(
        "300".to_string(),
        vec!["100".to_string(), "400".to_string()],
    );

    let resultado = recomendar_produtos(&grafo, "100");

    assert_eq!(
        resultado.iter().filter(|codigo| *codigo == "400").count(),
        1
    );
}

#[test]
fn respeita_limite_de_duas_conexoes() {
    let mut grafo: HashMap<String, Vec<String>> = HashMap::new();

    grafo.insert("100".to_string(), vec!["200".to_string()]);
    grafo.insert(
        "200".to_string(),
        vec!["100".to_string(), "300".to_string()],
    );
    grafo.insert(
        "300".to_string(),
        vec!["200".to_string(), "400".to_string()],
    );
    grafo.insert("400".to_string(), vec!["300".to_string()]);

    let resultado = recomendar_produtos(&grafo, "100");

    assert!(resultado.contains(&"200".to_string()));
    assert!(resultado.contains(&"300".to_string()));
    assert!(!resultado.contains(&"400".to_string()));
}

#[test]
fn cadastro_de_produtos_e_recomendacoes() {
    let estoque = Estoque::em_memoria().unwrap();

    estoque
        .cadastrar_produtos("Fone", "100", 50.0)
        .unwrap();

    estoque
        .cadastrar_produtos("Mouse", "200", 80.0)
        .unwrap();

    // Confirma que os produtos foram cadastrados.
    assert!(
        estoque.procurar_produto_codigo("100").unwrap().is_some()
    );

    assert!(
        estoque.procurar_produto_codigo("200").unwrap().is_some()
    );

    // Simula a conexão entre os códigos cadastrados.
    let mut grafo: HashMap<String, Vec<String>> = HashMap::new();

    grafo
        .entry("100".to_string())
        .or_default()
        .push("200".to_string());

    grafo
        .entry("200".to_string())
        .or_default()
        .push("100".to_string());

    let recomendacoes = recomendar_produtos(&grafo, "100");

    assert_eq!(recomendacoes, vec!["200".to_string()]);
}