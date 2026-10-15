use mega_store::Estoque;

#[test]
fn adicionar_produto() {
    let estoque = Estoque::em_memoria().unwrap();

    assert!(estoque.cadastrar_produtos("fone", "123", 10.0).is_ok());

    let produto = estoque.procurar_produto_codigo("123").unwrap();

    assert_eq!(
        produto,
        Some(("fone".to_string(), "123".to_string(), 10.0))
    );
}

#[test]
fn procurar_produto_existente() {
    let estoque = Estoque::em_memoria().unwrap();

    estoque.cadastrar_produtos("fone", "123", 10.0).unwrap();

    let produto = estoque.procurar_produto_codigo("123").unwrap();

    assert!(produto.is_some());
}

#[test]
fn procurar_produto_inexistente() {
    let estoque = Estoque::em_memoria().unwrap();

    let produto = estoque.procurar_produto_codigo("456").unwrap();

    assert_eq!(produto, None);
}

#[test]
fn cadastrar_produto_existente() {
    let estoque = Estoque::em_memoria().unwrap();

    estoque.cadastrar_produtos("fone", "123", 10.0).unwrap();

    let resultado =
        estoque.cadastrar_produtos("fone novo", "123", 20.0);

    assert!(resultado.is_err());
}

#[test]
fn remover_produto_existente() {
    let estoque = Estoque::em_memoria().unwrap();

    estoque.cadastrar_produtos("fone", "123", 10.0).unwrap();

    let removido = estoque.remover_produto("123").unwrap();

    assert!(removido);

    let produto = estoque.procurar_produto_codigo("123").unwrap();

    assert_eq!(produto, None);
}

#[test]
fn remover_produto_inexistente() {
    let estoque = Estoque::em_memoria().unwrap();

    let removido = estoque.remover_produto("456").unwrap();

    assert!(!removido);
}
