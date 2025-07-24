#[async_std::main]
async fn main() -> tide::Result<()> {
    // Define o endereco da porta API
    let addr  = "127.0.0.1:8080";

    println!("Servidor Tide rodando em : http://{}", addr);

    // Cria nova aplicação Tide
    let mut app = tide::new();

    //define rota GET para o caminho raiz ("/")
    // quando uma requisicao GET chega em "/", a funcao hello_world e chamada
    app.at("/").get(|_| async {
        Ok("Hello, World!")
    
    });

    // inicia o servidor tude e faz as requisicoes no addr definido
    app.listen(addr).await?;

    Ok(())
}
