use mega_store::recomendar_produtos;
use std::collections::HashMap;
use std::hint::black_box;
use std::time::Instant;

// Cria um grafo em formato de estrela.
// O vértice "0" está conectado a todos os outros.
fn criar_grafo_estrela(
    quantidade: usize,
) -> HashMap<String, Vec<String>> {
    let mut grafo: HashMap<String, Vec<String>> =
        HashMap::with_capacity(quantidade);

    let mut vizinhos = Vec::with_capacity(quantidade - 1);

    for i in 1..quantidade {
        let codigo = i.to_string();

        vizinhos.push(codigo.clone());

        grafo.insert(
            codigo,
            vec!["0".to_string()],
        );
    }

    grafo.insert("0".to_string(), vizinhos);

    grafo
}

fn main() {
    let tamanhos = [100, 1_000, 5_000, 10_000];
    let iteracoes = 200;

    println!("========================================");
    println!(" BENCHMARK BFS - MEGASTORE");
    println!("========================================");
    println!("Iterações por tamanho: {iteracoes}");
    println!("Grafo utilizado: estrela");
    println!();

    for tamanho in tamanhos {
        // A criação do grafo fica fora da medição.
        let grafo = criar_grafo_estrela(tamanho);

        // Aquecimento: executa a função antes da medição.
        for _ in 0..10 {
            black_box(recomendar_produtos(
                black_box(&grafo),
                black_box("0"),
            ));
        }

        let inicio = Instant::now();

        for _ in 0..iteracoes {
            let resultado = recomendar_produtos(
                black_box(&grafo),
                black_box("0"),
            );

            black_box(resultado);
        }

        let duracao = inicio.elapsed();

        let media_micros =
            duracao.as_secs_f64() * 1_000_000.0
            / iteracoes as f64;

        println!("Vértices: {tamanho:>6}");
        println!("Tempo total: {:.3} ms",
            duracao.as_secs_f64() * 1_000.0);
        println!("Média por execução: {:.3} µs",
            media_micros);
        println!("----------------------------------------");
    }
}