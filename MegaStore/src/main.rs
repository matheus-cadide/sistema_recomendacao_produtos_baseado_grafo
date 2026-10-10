use estoque::{cadastrar_produtos, criar_tbl_produtos, visualizar_produtos, procurar_produto_codigo};
use std::io;
fn main() -> rusqlite::Result<()> {
    criar_tbl_produtos()?;

    loop {
        let mut opcao = String::new();

        println!("=======================================\n
                Mega Store   \n
                1- Cadastrar Produtos\n
                2- Visualizar Produtos\n
                3- Procurar Produto pelo Código de Barras\n
                4- Sair\n
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
                    Ok(_) => println!(""),
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

                    match procurar_produto_codigo(&codigo) {
                        Ok(_) => println!(""),
                        Err(erro) => println!("Erro ao procurar produto: {}", erro),
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
            //Sair do programa
            "4" => {
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


