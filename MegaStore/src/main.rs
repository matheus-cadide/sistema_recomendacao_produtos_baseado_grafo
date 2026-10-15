use mega_store::{cadastrar_produtos, criar_tbl_produtos, visualizar_produtos, procurar_produto_codigo, recomendar_produtos};
use std::io;
use std::collections::{HashMap};

fn main() -> rusqlite::Result<()> {
    
    criar_tbl_produtos()?;
    let mut grafo: HashMap<String, Vec<String>> = HashMap::new();

    loop {
        let mut opcao = String::new();
        
        println!("=======================================\n
                Mega Store   \n
                1- Cadastrar Produtos\n
                2- Visualizar Produtos\n
                3- Procurar Produto pelo Código de Barras\n
                4- Fazer Conexão de Produtos\n
                5- Recomendações de Produtos\n
                6- Sair\n
                =======================================");


        io::stdin()
            .read_line(&mut opcao)
            .expect("Falha ao ler a opcao selecionada");
        
        match opcao.trim(){
            
            "1" => {
                println!("Digite o nome do produto:");
                let nome = ler_input();

                println!("Digite o código do produto:");
                let  codigo = ler_input();

                println!("Digite o preço do produto:");
                let  preco = ler_input().parse::<f64>().unwrap();

                match cadastrar_produtos(&nome, &codigo, preco) {
                    Ok(_) => println!("Produto cadastrado com sucesso!"),
                    Err(erro) => println!("Erro ao cadastrar produto: {}", erro),
                }
            }

            //Visualizar produtos

            "2" => {
                loop {
                    match visualizar_produtos() {

                        Ok(_) => println!("======================================"),
                        
                        Err(erro) => println!("Erro ao visualizar produtos: {}", erro),
                            
                        }
                

                    println!("Pressione Enter para sair da visualização de produtos.");

                    let  sair = ler_input();

                    match sair.trim() {

                    "" => {
                        println!("Saindo da visualização de produtos.");
                        break;

                    }

                    _ => println!("Erro ao sair da visualização de produtos: entrada inválida\n=====================================\n"),
                    }
                    
                }
                
                
            }


            //Procurar produto pelo código de barras


            "3" => {
                                
                'pesquisa:loop{
                    println!("=====================================");
                    println!("Procurar produto pelo código de barras");
                    println!("=====================================");
                    println!("Digite o código de barras do produto:");

                    let codigo = ler_input();
                    println!("=====================================");

                    match procurar_produto_codigo(codigo.trim()) {
                        Ok(Some((nome, codigo, preco))) => {
                            println!("Produto encontrado:");
                            println!("Nome: {}", nome);
                            println!("Código: {}", codigo);
                            println!("Preço: {}", preco);

                            println!("=====================================");
                        }
                        Ok(None) => {
                            println!("");
                            println!("======================================");
                            
                        }
                        Err(erro) => {
                            println!("Erro ao procurar produto: {}", erro);
                            println!("=====================================");
                        }
                        
                    }



                    loop{
                        println!("Digite 'sair' para sair da pesquisa ou pressione Enter para fazer outra pesquisa.");

                        let sair = ler_input();
                        match sair.trim() {
                        
                            //sair da pesquisa de produto
                            "sair" => {
                            
                                println!("Saindo da pesquisa de produto.");
                                break 'pesquisa;
                            
                            }
                            //fazer outra pesquisa
                            "" => {
                                println!("=====================================");
                                break;
                            }

                            //Outras entradas inválidas
                            _ => {
                                println!("Opção inválida");
                                println!("=====================================");
                                
                            
                            }
                        }
                    
                    }
                        
                    
                }
                
                
            }

            //Fazer conexão de produtos
            
            "4" => {
                loop {
                println!("Fazer conexão de produtos");
                println!("=====================================");
                
                println!("Digite o código do primeiro produto:");
                let codigo = ler_input();
                println!("=====================================");
                
                println!("Digite o código do produto a ser conectado:");
                let codigo_conectar = ler_input();
                println!("=====================================");
                
                //procurando produto pelo código de barras e conectando produtos
                
                //procurando o primeiro produto pelo código de barras
                match procurar_produto_codigo(&codigo) {

                    Ok(Some((nome, codigo_origem, _))) => {
                        
                        //procurando o segundo produto pelo código de barras se encontrou o primeiro produto
                        match procurar_produto_codigo(&codigo_conectar) {
                            
                            Ok(Some((nome_conectar, codigo_destino, _))) => {
                            
                                if codigo_origem == codigo_destino {
                                    println!("Não é possível conectar um produto a ele mesmo.");
                                    println!("=====================================");
                                
                                } else {

                                    // Cria ou recupera a lista de vizinhos
                                    
                                    let vizinhos_origem = grafo.entry(codigo_origem.clone()).or_default();

                                    if !vizinhos_origem.contains(&codigo_destino) {
                                        vizinhos_origem.push(codigo_destino.clone());

                                        // Cria a conexão no sentido inverso
                                        let vizinhos_destino = grafo.entry(codigo_destino.clone()).or_default();

                                        vizinhos_destino.push(codigo_origem.clone());

                                        println!(
                                            "{} conectado a {}!",
                                            nome, nome_conectar
                                        );
                                        println!("=====================================");
                                    } else {
                                        println!("Essa conexão já existe.");
                                        println!("=====================================");
                                    }
                                }
                            }
                            Ok(None) => {
                                println!("Segundo produto não encontrado.");
                                println!("=====================================");
                            }
                            Err(erro) => {
                                println!("Erro ao procurar segundo produto: {erro}");
                                println!("=====================================");
                            }
                        }
                    }
                    Ok(None) => {
                        println!("Primeiro produto não encontrado.");
                        println!("=====================================");
                    }
                    Err(erro) => {
                        println!("Erro ao procurar primeiro produto: {erro}");
                        println!("=====================================");
                    }
                }


                //Continuar ou encerrar conexões de produtos


                println!("Fazer outra conexão? (s/n)");

                let resposta = ler_input();
                
                
                match resposta.trim() {
                    "s" => {
                        println!("=====================================");
                        println!("Fazendo outra conexão de produtos.");
                        println!("=====================================");
                    }
                    "n" => {
                        println!("Encerrando conexões.");
                        println!("=====================================");
                        break;
                    }
                    _ => {
                        println!("Opção inválida. Encerrando conexões.");
                        println!("=====================================");
                        break;
                    }
                }
            }                                                
        }

            //Recomendações de produtos


            "5" => {
                println!("=====================================");
                println!("Recomendações de produtos");
                println!("Digite o código do produto:");
                
                let codigo = ler_input();

                match procurar_produto_codigo(&codigo) {
                    Ok(Some((nome, codigo_produto, _))) => {
                        let recomendacoes =
                            recomendar_produtos(&grafo, &codigo_produto);

                        if recomendacoes.is_empty() {
                            println!(
                                "Não há recomendações para o produto {}.",
                                nome
                            );
                        } else {
                            println!(
                                "\nProdutos recomendados para {}:",
                                nome
                            );

                            for codigo_recomendado in recomendacoes {
                                match procurar_produto_codigo(&codigo_recomendado) {
                                    Ok(Some((nome, codigo, preco))) => {
                                        println!(
                                            "- {} | Código: {} | Preço: R$ {:.2}",
                                            nome, codigo, preco
                                        );
                                    }
                                    Ok(None) => {
                                        println!(
                                            "Produto {} não encontrado no banco.",
                                            codigo_recomendado
                                        );
                                    }
                                    Err(erro) => {
                                        println!(
                                            "Erro ao consultar recomendação: {}",
                                            erro
                                        );
                                    }
                                }
                            }
                        }
                    }
                    Ok(None) => {
                        println!("Produto não encontrado.");
                    }
                    Err(erro) => {
                        println!("Erro ao procurar produto: {}", erro);
                    }
                }                 
            }
 
            //Sair do programa
            "6" => {
                println!("Saindo...");
                break;
            }
            _ => {
                println!("Opção inválida");
            }
        }
            
        

    }
    Ok(())
} 
        


fn ler_input() -> String {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Falha ao ler o dado");
    input.trim().to_string()
}

